use embassy_futures::join::join;
use embassy_rp::gpio::Input;
use embassy_time::Instant;
use sb_rotary_encoder::RotaryEncoder as RE;

pub struct RotaryEncoder {
    encoder: RE,
    input_a: Input<'static>,
    input_b: Input<'static>,
}

impl RotaryEncoder {
    pub fn new(input_a: Input<'static>, input_b: Input<'static>) -> Self {
        let encoder = RE::new();

        Self {
            input_a,
            input_b,
            encoder,
        }
    }

    pub async fn on_change(&mut self) -> (i32, i32) {
        loop {
            let a_future = self.input_a.wait_for_any_edge();
            let b_future = self.input_b.wait_for_any_edge();
            join(a_future, b_future).await;
            let tick = Instant::now().as_ticks();

            let maybe_event = self.encoder.update(
                self.input_a.is_high(),
                self.input_b.is_high(),
                Some(tick),
                4,
            );

            if let Some(event) = maybe_event {
                break (event.step(), 0);
                // TODO: figure out velocity....
                //event.velocity(100);
            }
        }
    }
}
