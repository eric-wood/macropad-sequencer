use core::sync::atomic::Ordering;

use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
use smart_leds::RGB;

use crate::{
    COLS, KeyGrid, ROWS,
    key_leds::Coord,
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
}

#[embassy_executor::task]
pub async fn read_controls() {
    let mut key_state: KeyGrid<bool> = [[false; COLS]; ROWS];

    let active = RGB { r: 5, g: 5, b: 5 };
    let off = RGB { r: 0, g: 0, b: 0 };
    let current = RGB { r: 32, g: 0, b: 0 };

    let mut step: Coord = (0, 0);

    loop {
        match CONTROLS_CHANNEL.receive().await {
            ControlEvent::Key { pressed, coord } => {
                if !pressed {
                    continue;
                }

                let mut state = key_state[coord.1 as usize][coord.0 as usize];
                state = !state;
                key_state[coord.1 as usize][coord.0 as usize] = state;
                let color = if state { active } else { off };
                update_key_light(coord, color).await;
            }
            ControlEvent::RotaryButton { pressed } => {
                if !pressed {
                    continue;
                }

                rotary_press().await;
            }
            ControlEvent::RotaryEncoder { increment } => rotary_change(increment).await,
            ControlEvent::SequencerStep { coord } => {
                let prev_color = if key_state[step.1 as usize][step.0 as usize] {
                    active
                } else {
                    off
                };

                update_key_light(step, prev_color).await;
                update_key_light(coord, current).await;
                step = coord;
            }
        }
    }
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
