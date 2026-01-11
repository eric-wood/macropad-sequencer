use crate::{
    menus::{SEQUENCER_MENU, SequencerMenuValue},
    sequencer_timer::{SequencerConfig, SequencerTimer},
    tasks::{CONTROLS_CHANNEL, controls::ControlEvent},
};

#[embassy_executor::task]
pub async fn sequencer() {
    let mut timer = SequencerTimer::new();
    let mut play = false;
    loop {
        SEQUENCER_MENU.lock(|value| {
            let SequencerMenuValue {
                play: temp_play,
                bpm,
                timing,
                steps: _steps,
            } = value.unwrap();
            timer.set(SequencerConfig { bpm, timing });
            play = temp_play;
        });

        if play {
            CONTROLS_CHANNEL.send(ControlEvent::SequencerStep).await;
        }

        timer.next_step().await;
    }
}
