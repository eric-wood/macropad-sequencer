use embassy_rp::{
    peripherals::PIO0,
    pio_programs::rotary_encoder::{Direction, PioEncoder},
};

pub struct RotaryEncoder {
    encoder: PioEncoder<'static, PIO0, 0>,
}

impl RotaryEncoder {
    pub fn new(encoder: PioEncoder<'static, PIO0, 0>) -> Self {
        Self { encoder }
    }

    pub async fn on_change(&mut self) -> (i32, i32) {
        let step = match self.encoder.read().await {
            Direction::Clockwise => 1,
            Direction::CounterClockwise => -1,
        };

        (step, 0)
    }
}
