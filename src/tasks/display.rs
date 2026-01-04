use core::sync::atomic::Ordering;

use crate::{
    BPM, PLAY, SWING, TIMING,
    menus::{BooleanMenuItem, EnumMenuItem, Menu, NumericMenuItem, SequencerMenu},
    sequencer_timer::TimingOption,
};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};

use crate::display::Display;

pub static DISPLAY_CHANNEL: Channel<ThreadModeRawMutex, DisplayUpdate, 3> = Channel::new();

pub enum DisplayUpdate {
    RotaryMove { increment: i32 },
    RotaryPress,
}

struct Menus<'a> {
    pub sequencer: SequencerMenu<'a, 4>,
}

impl<'a> Menus<'a> {
    pub fn new(sequencer: SequencerMenu<'a, 4>) -> Self {
        Self { sequencer }
    }
}

#[embassy_executor::task]
pub async fn drive_display(mut display: Display) {
    display.init();
    TIMING.store(TimingOption::Quarter.into(), Ordering::Relaxed);
    let mut play_menu = BooleanMenuItem::new("STATUS", "PLAYING", "PAUSED", &PLAY);
    let mut bpm_menu = NumericMenuItem::new("BPM", &BPM);
    let mut timing_menu = EnumMenuItem::new(
        "TIMING",
        [
            TimingOption::Quarter,
            TimingOption::QuarterTriplet,
            TimingOption::Eighth,
            TimingOption::EighthTriplet,
            TimingOption::Sixteenth,
            TimingOption::SixteenthTriplet,
        ],
        &TIMING,
    );
    let mut swing_menu = NumericMenuItem::new("SWING", &SWING);

    let sequencer = SequencerMenu::new([
        &mut play_menu,
        &mut bpm_menu,
        &mut timing_menu,
        &mut swing_menu,
    ]);

    let mut menus = Menus::new(sequencer);
    menus.sequencer.render(&mut display.display);
    display.flush();

    loop {
        match DISPLAY_CHANNEL.receive().await {
            DisplayUpdate::RotaryMove { increment } => {
                menus.sequencer.on_change(increment);
            }
            DisplayUpdate::RotaryPress => {
                menus.sequencer.on_select();
            }
        }

        display.clear();
        menus.sequencer.render(&mut display.display);
        display.flush();
    }
}
