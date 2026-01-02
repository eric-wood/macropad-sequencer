use crate::{
    rotary_encoder::RotaryEncoder,
    tasks::{CONTROLS_CHANNEL, controls::ControlEvent},
};

#[embassy_executor::task]
pub async fn read_rotary_encoder(mut encoder: RotaryEncoder) {
    loop {
        let (increment, _) = encoder.on_change().await;

        CONTROLS_CHANNEL
            .send(ControlEvent::RotaryEncoder { increment })
            .await;
    }
}
