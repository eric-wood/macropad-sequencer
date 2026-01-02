use embassy_rp::gpio::Input;
use embassy_time::{Duration, Timer};

pub struct DebouncedButton<'a> {
    pub threshold: Duration,
    input: Input<'a>,
}

impl<'a> DebouncedButton<'a> {
    pub fn new(input: Input<'a>, threshold: Duration) -> Self {
        DebouncedButton { threshold, input }
    }

    pub async fn on_change(&mut self) -> bool {
        loop {
            let l1 = self.pressed();
            self.input.wait_for_any_edge().await;
            Timer::after(self.threshold).await;
            let l2 = self.pressed();
            if l1 != l2 {
                break l2;
            }
        }
    }

    fn pressed(&mut self) -> bool {
        self.input.is_low()
    }
}
