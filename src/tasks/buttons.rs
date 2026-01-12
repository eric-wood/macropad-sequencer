use embassy_rp::gpio::Input;
use embassy_time::Duration;

use crate::{
    debounced_button::DebouncedButton,
    key_leds::Coord,
    tasks::controls::{CONTROLS_CHANNEL, ControlEvent},
    toggle_with_hold::ToggleWithHold,
};

#[embassy_executor::task(pool_size = 12)]
pub async fn read_key(input: Input<'static>, coord: Coord) {
    let button = DebouncedButton::new(input, Duration::from_millis(10));
    let mut toggle = ToggleWithHold::new(button, Duration::from_millis(300));

    loop {
        toggle.on_change().await;
        CONTROLS_CHANNEL
            .send(ControlEvent::Key {
                pressed: toggle.is_pressed,
                held: toggle.is_held,
                coord,
            })
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
