use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
use smart_leds::RGB;

use crate::key_leds::{Coord, KeyLeds};

pub static LIGHTS_CHANNEL: Channel<ThreadModeRawMutex, LedUpdate, 3> = Channel::new();

pub struct LedUpdate {
    pub coord: Coord,
    pub color: RGB<u8>,
}

#[embassy_executor::task]
pub async fn update_lights(mut key_leds: KeyLeds) {
    loop {
        let led_update = LIGHTS_CHANNEL.receive().await;
        key_leds.set(led_update.coord, led_update.color);
        key_leds.write();
    }
}
