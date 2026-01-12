use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
use smart_leds::RGB;

use crate::{
    COLS, KeyGrid, NUM_KEYS, ROWS,
    key_leds::Coord,
    menus::{SEQUENCER_MENU, SequencerMenuValue, StepMenuValue},
    tasks::{
        display::{DISPLAY_CHANNEL, DisplayUpdate},
        lights::{LIGHTS_CHANNEL, LedUpdate},
        usb_midi::{MIDI_CHANNEL, MidiEvent},
    },
};

pub static CONTROLS_CHANNEL: Channel<ThreadModeRawMutex, ControlEvent, 10> = Channel::new();

pub enum ControlEvent {
    Key {
        pressed: bool,
        held: bool,
        coord: Coord,
    },
    RotaryButton {
        pressed: bool,
    },
    SequencerStep,
    RotaryEncoder {
        increment: i32,
    },
    SequencerMenuChange {
        value: SequencerMenuValue,
    },
    StepMenuChange {
        value: StepMenuValue,
    },
}

#[derive(Default, Clone, Copy)]
struct StepState {
    active: bool,
    pressed: bool,
    held: bool,
    value: StepMenuValue,
}

#[embassy_executor::task]
pub async fn read_controls() {
    let mut step_state: KeyGrid<StepState> = [[StepState::default(); COLS]; ROWS];

    let active = RGB { r: 5, g: 5, b: 5 };
    let off = RGB { r: 0, g: 0, b: 0 };
    let current = RGB { r: 32, g: 0, b: 0 };

    let mut num_steps = 12;
    let mut num_keys_pressed = 0;
    let mut selected_step: Option<Coord> = None;
    let mut step_index: usize = 0;
    let mut step: Coord = (0, 0);
    let mut last_note: Option<StepMenuValue> = None;
    let mut play = false;

    loop {
        match CONTROLS_CHANNEL.receive().await {
            ControlEvent::Key {
                pressed,
                held,
                coord,
            } => {
                let state = &mut step_state[coord.1 as usize][coord.0 as usize];
                let was_pressed = state.pressed;
                let was_held = state.held;
                state.pressed = pressed;
                state.held = held;
                if !pressed && was_pressed && !was_held {
                    state.active = !state.active;
                    let color = if state.active { active } else { off };
                    update_key_light(coord, color).await;
                }

                if !pressed {
                    num_keys_pressed -= 1;
                    if num_keys_pressed != 1 {
                        set_step_menu(None).await;
                    }
                } else {
                    if !was_pressed && pressed {
                        num_keys_pressed += 1;
                    }
                    let value = if num_keys_pressed == 1 && held {
                        selected_step = Some(coord);
                        Some(state.value)
                    } else {
                        selected_step = None;
                        None
                    };

                    set_step_menu(value).await;
                }
            }
            ControlEvent::RotaryButton { pressed } => {
                if !pressed {
                    continue;
                }

                rotary_press().await;
            }
            ControlEvent::RotaryEncoder { increment } => rotary_change(increment).await,
            ControlEvent::SequencerStep => {
                let state = step_state[step.1 as usize][step.0 as usize];

                if let Some(value) = last_note {
                    send_note(false, value).await;
                }

                let prev_color = if state.active { active } else { off };

                let next_step = if num_keys_pressed > 0 {
                    step_index = (step_index + 1).rem_euclid(NUM_KEYS);
                    let mut next = coord_from_index(step_index);
                    while !step_state[next.1 as usize][next.0 as usize].pressed {
                        step_index = (step_index + 1).rem_euclid(NUM_KEYS);
                        next = coord_from_index(step_index)
                    }
                    next
                } else {
                    step_index = (step_index + 1).rem_euclid(num_steps);
                    coord_from_index(step_index)
                };

                let state = step_state[next_step.1 as usize][next_step.0 as usize];
                last_note = Some(state.value);

                if state.active || num_keys_pressed > 0 {
                    send_note(true, state.value).await;
                }

                update_key_light(step, prev_color).await;
                update_key_light(next_step, current).await;
                step = next_step;
            }
            ControlEvent::SequencerMenuChange { value } => unsafe {
                num_steps = value.steps as usize;
                if play
                    && !value.play
                    && let Some(note) = last_note
                {
                    send_note(false, note).await;
                }
                play = value.play;
                SEQUENCER_MENU.lock_mut(|inner| *inner = Some(value));
            },
            ControlEvent::StepMenuChange { value } => {
                if let Some(coord) = selected_step {
                    step_state[coord.1 as usize][coord.0 as usize].value = value;
                }
            }
        }
    }
}

fn coord_from_index(index: usize) -> Coord {
    ((index % COLS) as u8, (index / COLS) as u8)
}

async fn send_note(on: bool, value: StepMenuValue) {
    MIDI_CHANNEL
        .send(MidiEvent::Note {
            on,
            note: value.note,
            octave: value.octave as u8,
            velocity: value.velocity as u8,
        })
        .await;
}

async fn set_step_menu(value: Option<StepMenuValue>) {
    DISPLAY_CHANNEL
        .send(DisplayUpdate::StepMenu { value })
        .await;
}

async fn rotary_press() {
    DISPLAY_CHANNEL.send(DisplayUpdate::RotaryPress).await;
}

async fn rotary_change(increment: i32) {
    DISPLAY_CHANNEL
        .send(DisplayUpdate::RotaryMove { increment })
        .await;
}

async fn update_key_light(coord: Coord, color: RGB<u8>) {
    LIGHTS_CHANNEL.send(LedUpdate { coord, color }).await;
}
