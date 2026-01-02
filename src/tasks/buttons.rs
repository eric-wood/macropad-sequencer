use embassy_rp::gpio::Input;
use embassy_time::Duration;

use crate::{
    debounced_button::DebouncedButton,
    key_leds::Coord,
    tasks::controls::{CONTROLS_CHANNEL, ControlEvent},
};

#[embassy_executor::task(pool_size = 12)]
pub async fn read_key(input: Input<'static>, coord: Coord) {
    let mut button = DebouncedButton::new(input, Duration::from_millis(10));

    loop {
        let pressed = button.on_change().await;
        CONTROLS_CHANNEL
            .send(ControlEvent::Key { pressed, coord })
            .await;
    }
}

#[embassy_executor::task]
pub async fn read_button(input: Input<'static>) {
    let mut button = DebouncedButton::new(input, Duration::from_millis(10));

    loop {
        let pressed = button.on_change().await;
        CONTROLS_CHANNEL
            .send(ControlEvent::RotaryButton { pressed })
            .await;
    }
}
