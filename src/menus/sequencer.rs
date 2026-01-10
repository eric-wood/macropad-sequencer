use core::cell::RefCell;

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
    pub swing: u32,
}

impl Default for SequencerMenuValue {
    fn default() -> Self {
        Self {
            play: false,
            bpm: 120,
            timing: TimingOption::Eighth,
            swing: 0,
        }
    }
}

type SequencerMenuMutex = Mutex<ThreadModeRawMutex, Option<SequencerMenuValue>>;
pub static SEQUENCER_MENU: SequencerMenuMutex = Mutex::new(None);

pub struct SequencerMenuItems<'a> {
    pub play_menu: BooleanMenuItem<'a>,
    pub bpm_menu: NumericMenuItem<'a>,
    pub timing_menu: EnumMenuItem<'a, 6, TimingOption>,
    pub swing_menu: NumericMenuItem<'a>,
}

impl<'a> SequencerMenuItems<'a> {
    pub fn new() -> Self {
        let defaults = SequencerMenuValue::default();
        let play_menu =
            BooleanMenuItem::new("STATUS", "PLAYING", "PAUSED", defaults.play, &|value| {
                unsafe {
                    SEQUENCER_MENU.lock_mut(|inner| {
                        if let Some(menu_value) = inner {
                            menu_value.play = value;
                        }
                    })
                };
            });

        let bpm_menu = NumericMenuItem::new("BPM", defaults.bpm, &|value| {
            unsafe {
                SEQUENCER_MENU.lock_mut(|inner| {
                    if let Some(menu_value) = inner {
                        menu_value.bpm = value;
                    }
                })
            };
        });

        let timing_menu = EnumMenuItem::new(
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
            &|value| {
                unsafe {
                    SEQUENCER_MENU.lock_mut(|inner| {
                        if let Some(menu_value) = inner {
                            menu_value.timing = value;
                        }
                    })
                };
            },
        );
        let swing_menu = NumericMenuItem::new("SWING", defaults.swing, &|value| {
            unsafe {
                SEQUENCER_MENU.lock_mut(|inner| {
                    if let Some(menu_value) = inner {
                        menu_value.swing = value;
                    }
                })
            };
        });

        unsafe {
            SEQUENCER_MENU.lock_mut(|value| {
                *value = Some(defaults);
            });
        }

        Self {
            play_menu,
            bpm_menu,
            timing_menu,
            swing_menu,
        }
    }
}
