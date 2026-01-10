use crate::{
    COLS,
    menus::{SEQUENCER_MENU, SequencerMenuValue},
    sequencer_timer::{SequencerConfig, SequencerTimer},
    tasks::{CONTROLS_CHANNEL, controls::ControlEvent},
};

#[embassy_executor::task]
pub async fn sequencer() {
    let mut step: u8 = 0;
    let cols = COLS as u8;

    let mut timer = SequencerTimer::new();
    let mut play = false;
    loop {
        SEQUENCER_MENU.lock(|value| {
            let SequencerMenuValue {
                play: temp_play,
                bpm,
                timing,
                swing,
            } = value.unwrap();
            timer.set(SequencerConfig { bpm, timing, swing });
            play = temp_play;
        });

        if play {
            let coord = (step % cols, step / cols);
            CONTROLS_CHANNEL
                .send(ControlEvent::SequencerStep { coord })
                .await;

            step = (step + 1).rem_euclid(12);
        }

        timer.next_step().await;
    }
}
