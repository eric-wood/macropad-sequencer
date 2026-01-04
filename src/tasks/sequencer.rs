use core::sync::atomic::Ordering;

use crate::{
    BPM, COLS, PLAY, SWING, TIMING,
    sequencer_timer::{SequencerConfig, SequencerTimer},
    tasks::{CONTROLS_CHANNEL, controls::ControlEvent},
};

#[embassy_executor::task]
pub async fn sequencer() {
    let mut step: u8 = 0;
    let cols = COLS as u8;

    let mut timer = SequencerTimer::new();
    loop {
        let play = PLAY.load(Ordering::Relaxed);
        if play {
            let coord = (step % cols, step / cols);
            CONTROLS_CHANNEL
                .send(ControlEvent::SequencerStep { coord })
                .await;

            step = (step + 1).rem_euclid(12);

            timer.set(SequencerConfig {
                bpm: BPM.load(Ordering::Relaxed),
                timing: TIMING.load(Ordering::Relaxed).into(),
                swing: SWING.load(Ordering::Relaxed),
            });
        }

        timer.next_step().await;
    }
}
