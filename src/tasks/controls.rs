use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
use smart_leds::RGB;

use crate::{
    COLS, KeyGrid, ROWS,
    key_leds::Coord,
    menus::{SEQUENCER_MENU, SequencerMenuValue, StepMenuValue},
    tasks::{
        display::{DISPLAY_CHANNEL, DisplayUpdate},
        lights::{LIGHTS_CHANNEL, LedUpdate},
    },
};

pub static CONTROLS_CHANNEL: Channel<ThreadModeRawMutex, ControlEvent, 10> = Channel::new();

pub enum ControlEvent {
    Key { pressed: bool, coord: Coord },
    RotaryButton { pressed: bool },
    SequencerStep { coord: Coord },
    RotaryEncoder { increment: i32 },
    SequencerMenuChange { value: SequencerMenuValue },
    StepMenuChange { value: StepMenuValue },
}

#[derive(Default, Clone, Copy)]
struct StepState {
    active: bool,
    pressed: bool,
    value: StepMenuValue,
}

#[embassy_executor::task]
pub async fn read_controls() {
    let mut step_state: KeyGrid<StepState> = [[StepState::default(); COLS]; ROWS];

    let active = RGB { r: 5, g: 5, b: 5 };
    let off = RGB { r: 0, g: 0, b: 0 };
    let current = RGB { r: 32, g: 0, b: 0 };

    let mut num_keys_pressed = 0;
    let mut selected_step: Option<Coord> = None;
    let mut step: Coord = (0, 0);

    loop {
        match CONTROLS_CHANNEL.receive().await {
            ControlEvent::Key { pressed, coord } => {
                let state = &mut step_state[coord.1 as usize][coord.0 as usize];

                if !pressed {
                    state.pressed = false;
                    num_keys_pressed -= 1;
                    if num_keys_pressed != 1 {
                        set_step_menu(None).await;
                    }
                    continue;
                }

                state.pressed = true;
                state.active = !state.active;
                let color = if state.active { active } else { off };
                update_key_light(coord, color).await;

                num_keys_pressed += 1;
                let value = if num_keys_pressed == 1 {
                    selected_step = Some(coord);
                    Some(state.value)
                } else {
                    selected_step = None;
                    None
                };
                set_step_menu(value).await;
            }
            ControlEvent::RotaryButton { pressed } => {
                if !pressed {
                    continue;
                }

                rotary_press().await;
            }
            ControlEvent::RotaryEncoder { increment } => rotary_change(increment).await,
            ControlEvent::SequencerStep { coord } => {
                let prev_color = if step_state[step.1 as usize][step.0 as usize].active {
                    active
                } else {
                    off
                };

                update_key_light(step, prev_color).await;
                update_key_light(coord, current).await;
                step = coord;
            }
            ControlEvent::SequencerMenuChange { value } => unsafe {
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
