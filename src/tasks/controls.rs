use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
use smart_leds::RGB;

use crate::{
    COLS, KeyGrid, ROWS,
    key_leds::Coord,
    tasks::lights::{LIGHTS_CHANNEL, LedUpdate},
};

pub static CONTROLS_CHANNEL: Channel<ThreadModeRawMutex, ControlEvent, 10> = Channel::new();

pub enum ControlEvent {
    Key { pressed: bool, coord: Coord },
    RotaryButton { pressed: bool },
    SequencerStep { coord: Coord },
}

#[embassy_executor::task]
pub async fn read_controls() {
    let mut key_state: KeyGrid<bool> = [[false; COLS]; ROWS];
    let active = RGB {
        r: 0x40,
        g: 0x00,
        b: 0x00,
    };

    let off = RGB { r: 0, g: 0, b: 0 };
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
                LIGHTS_CHANNEL.send(LedUpdate { coord, color }).await;
            }
            ControlEvent::RotaryButton { pressed } => {
                if !pressed {
                    continue;
                }

                for y in 0..4 {
                    for x in 0..3 {
                        LIGHTS_CHANNEL
                            .send(LedUpdate {
                                coord: (x, y),
                                color: RGB {
                                    r: 0x40,
                                    g: 0,
                                    b: 0,
                                },
                            })
                            .await;
                    }
                }
            }
            ControlEvent::SequencerStep { coord } => {
                LIGHTS_CHANNEL
                    .send(LedUpdate {
                        coord: step,
                        color: RGB { r: 0, g: 0, b: 0 },
                    })
                    .await;
                LIGHTS_CHANNEL
                    .send(LedUpdate {
                        coord,
                        color: RGB {
                            r: 0,
                            g: 0,
                            b: 0x50,
                        },
                    })
                    .await;
                step = coord;
            }
        }
    }
}
