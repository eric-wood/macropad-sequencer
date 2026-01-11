use embassy_sync::blocking_mutex::{Mutex, raw::ThreadModeRawMutex};

use crate::{
    menus::{BooleanMenuItem, EnumMenuItem, NumericMenuItem, Stringable},
    sequencer_timer::TimingOption,
};

impl Stringable for TimingOption {
    fn as_str(&self) -> &str {
        match self {
            TimingOption::Quarter => "1/4",
            TimingOption::QuarterTriplet => "1/4 triplet",
            TimingOption::Eighth => "1/8",
            TimingOption::EighthTriplet => "1/8 triplet",
            TimingOption::Sixteenth => "1/16",
            TimingOption::SixteenthTriplet => "1/16 triplet",
        }
    }
}

#[derive(Clone, Copy)]
pub struct SequencerMenuValue {
    pub play: bool,
    pub bpm: u32,
    pub timing: TimingOption,
    pub steps: u32,
}

impl Default for SequencerMenuValue {
    fn default() -> Self {
        Self {
            play: false,
            bpm: 120,
            timing: TimingOption::Eighth,
            steps: 12,
        }
    }
}

type SequencerMenuMutex = Mutex<ThreadModeRawMutex, Option<SequencerMenuValue>>;
pub static SEQUENCER_MENU: SequencerMenuMutex = Mutex::new(None);

pub struct SequencerMenuItems<'a> {
    pub play_menu: BooleanMenuItem<'a, SequencerMenuValue>,
    pub bpm_menu: NumericMenuItem<'a, SequencerMenuValue>,
    pub timing_menu: EnumMenuItem<'a, SequencerMenuValue, 6, TimingOption>,
    pub steps_menu: NumericMenuItem<'a, SequencerMenuValue>,
}

impl<'a> SequencerMenuItems<'a> {
    pub fn new() -> Self {
        let defaults = SequencerMenuValue::default();
        let play_menu = BooleanMenuItem::<SequencerMenuValue>::new(
            "STATUS",
            "PLAYING",
            "PAUSED",
            defaults.play,
            &|menu_value, value| {
                menu_value.play = value;
            },
        );

        let bpm_menu = NumericMenuItem::<SequencerMenuValue>::new(
            "BPM",
            defaults.bpm,
            &|menu_value, value| {
                menu_value.bpm = value;
            },
        );

        let timing_menu = EnumMenuItem::<'_, SequencerMenuValue, 6, TimingOption>::new(
            "TIMING",
            [
                TimingOption::Quarter,
                TimingOption::QuarterTriplet,
                TimingOption::Eighth,
                TimingOption::EighthTriplet,
                TimingOption::Sixteenth,
                TimingOption::SixteenthTriplet,
            ],
            defaults.timing,
            &|menu_value, value| {
                menu_value.timing = value;
            },
        );
        let steps_menu = NumericMenuItem::<SequencerMenuValue>::new(
            "STEPS",
            defaults.steps,
            &|menu_value, value| {
                menu_value.steps = value;
            },
        );

        unsafe {
            SEQUENCER_MENU.lock_mut(|value| {
                *value = Some(defaults);
            });
        }

        Self {
            play_menu,
            bpm_menu,
            timing_menu,
            steps_menu,
        }
    }
}
