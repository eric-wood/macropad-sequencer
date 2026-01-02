use core::sync::atomic::Ordering;

use embassy_time::Timer;

use crate::{
    COLS, SPEED_MS,
    tasks::{CONTROLS_CHANNEL, controls::ControlEvent},
};

#[embassy_executor::task]
pub async fn sequencer() {
    let mut step: u8 = 0;
    let cols = COLS as u8;

    loop {
        let coord = (step % cols, step / cols);
        CONTROLS_CHANNEL
            .send(ControlEvent::SequencerStep { coord })
            .await;

        step += 1;
        if step == 12 {
            step = 0;
        }

        Timer::after_millis(SPEED_MS.load(Ordering::Relaxed) as u64).await;
    }
}
