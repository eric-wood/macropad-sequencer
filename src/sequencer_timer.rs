use embassy_time::Timer;

use crate::{COLS, ROWS};

#[derive(Clone, Copy, Default, PartialEq)]
pub enum TimingOption {
    #[default]
    Quarter,
    QuarterTriplet,
    Eighth,
    EighthTriplet,
    Sixteenth,
    SixteenthTriplet,
}

const NUM_STEPS: usize = ROWS * COLS;

pub struct SequencerConfig {
    pub bpm: u32,
    pub timing: TimingOption,
    pub swing: u32,
}

pub struct SequencerTimer {
    config: SequencerConfig,
    speed_ms: u32,
}

impl SequencerTimer {
    pub fn new() -> Self {
        let config = SequencerConfig {
            bpm: 120,
            timing: TimingOption::Quarter,
            swing: 0,
        };

        let speed_ms = config_to_ms(&config);

        Self { config, speed_ms }
    }

    pub fn set(&mut self, config: SequencerConfig) {
        self.config = config;
        self.speed_ms = config_to_ms(&self.config);
    }

    pub async fn next_step(&mut self) {
        Timer::after_millis(self.speed_ms as u64).await;
    }
}

fn config_to_ms(config: &SequencerConfig) -> u32 {
    // TODO: swing
    // TODO: does this need to be µS to preserve timing?
    match config.timing {
        TimingOption::Quarter => 60_000 / config.bpm,
        TimingOption::QuarterTriplet => 40_000 / config.bpm,
        TimingOption::Eighth => 30_000 / config.bpm,
        TimingOption::EighthTriplet => 20_000 / config.bpm,
        TimingOption::Sixteenth => 15_000 / config.bpm,
        TimingOption::SixteenthTriplet => 10_000 / config.bpm,
    }
}
