use crate::menus::{
    BooleanMenuItem, EnumMenuItem, Menu, NumericMenuItem, SequencerMenu, Stringable,
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

#[derive(Clone, Copy)]
pub enum TimingOption {
    Quarter,
    Eighth,
    Sixteenth,
}

impl Stringable for TimingOption {
    fn as_str(&self) -> &str {
        match self {
            TimingOption::Quarter => "1/4",
            TimingOption::Eighth => "1/8",
            TimingOption::Sixteenth => "1/16",
        }
    }
}

#[embassy_executor::task]
pub async fn drive_display(mut display: Display) {
    display.init();

    let mut play_menu = BooleanMenuItem::new("STATUS", "PLAYING", "PAUSED");
    let mut bpm_menu = NumericMenuItem::new("BPM", 120);
    let mut timing_menu = EnumMenuItem::new(
        "TIMING",
        [
            TimingOption::Quarter,
            TimingOption::Eighth,
            TimingOption::Sixteenth,
        ],
    );
    let mut swing_menu = NumericMenuItem::new("SWING", 0);

    let sequencer = SequencerMenu::new([
        &mut play_menu,
        &mut bpm_menu,
        &mut timing_menu,
        &mut swing_menu,
    ]);

    let mut menus = Menus::new(sequencer);

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
