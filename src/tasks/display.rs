use crate::{
    menus::{Menu, SequencerMenuItems, SequencerMenuValue, StepMenuItems, StepMenuValue},
    tasks::{CONTROLS_CHANNEL, ControlEvent},
};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};

use crate::display::Display;

pub static DISPLAY_CHANNEL: Channel<ThreadModeRawMutex, DisplayUpdate, 3> = Channel::new();

pub enum DisplayUpdate {
    RotaryMove { increment: i32 },
    RotaryPress,
    StepMenu { value: Option<StepMenuValue> },
}

#[embassy_executor::task]
pub async fn drive_display(mut display: Display) {
    display.init();

    let mut sequencer_items = SequencerMenuItems::new();
    let mut sequencer_menu = Menu::new(
        "Sequencer",
        SequencerMenuValue::default(),
        [
            &mut sequencer_items.play_menu,
            &mut sequencer_items.bpm_menu,
            &mut sequencer_items.timing_menu,
            &mut sequencer_items.steps_menu,
        ],
        &|value| {
            let _ = CONTROLS_CHANNEL.try_send(ControlEvent::SequencerMenuChange { value: *value });
        },
    );

    let mut step_items = StepMenuItems::new();
    let mut step_menu = Menu::new(
        "Step",
        StepMenuValue::default(),
        [
            &mut step_items.note_menu,
            &mut step_items.octave_menu,
            &mut step_items.velocity_menu,
        ],
        &|_| {},
    );

    sequencer_menu.render(&mut display.display);
    display.flush();
    let mut show_step = false;

    loop {
        match DISPLAY_CHANNEL.receive().await {
            DisplayUpdate::RotaryMove { increment } => {
                if show_step {
                    step_menu.on_change(increment).await;
                } else {
                    sequencer_menu.on_change(increment).await;
                }
            }
            DisplayUpdate::RotaryPress => {
                if show_step {
                    step_menu.on_select();
                } else {
                    sequencer_menu.on_select();
                }
            }
            DisplayUpdate::StepMenu { value } => {
                show_step = value.is_some();
                if let Some(value) = value {
                    step_items = StepMenuItems::new();
                    step_items.note_menu.set(value.note);
                    step_items.octave_menu.value = value.octave;
                    step_items.velocity_menu.value = value.velocity;
                    step_menu = Menu::new(
                        "Step",
                        StepMenuValue::default(),
                        [
                            &mut step_items.note_menu,
                            &mut step_items.octave_menu,
                            &mut step_items.velocity_menu,
                        ],
                        &|value| {
                            let _ = CONTROLS_CHANNEL
                                .try_send(ControlEvent::StepMenuChange { value: *value });
                        },
                    );
                }
            }
        }

        display.clear();
        if show_step {
            step_menu.render(&mut display.display);
        } else {
            sequencer_menu.render(&mut display.display);
        }
        display.flush();
    }
}
