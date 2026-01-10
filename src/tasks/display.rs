use crate::menus::{Menu, SequencerMenuItems};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};

use crate::display::Display;

pub static DISPLAY_CHANNEL: Channel<ThreadModeRawMutex, DisplayUpdate, 3> = Channel::new();

pub enum DisplayUpdate {
    RotaryMove { increment: i32 },
    RotaryPress,
}

struct Menus<'a> {
    pub sequencer: Menu<'a, 4>,
}

impl<'a> Menus<'a> {
    pub fn new(sequencer: Menu<'a, 4>) -> Self {
        Self { sequencer }
    }
}

#[embassy_executor::task]
pub async fn drive_display(mut display: Display) {
    display.init();

    let mut sequencer_items = SequencerMenuItems::new();
    let sequencer = Menu::new(
        "Sequencer",
        [
            &mut sequencer_items.play_menu,
            &mut sequencer_items.bpm_menu,
            &mut sequencer_items.timing_menu,
            &mut sequencer_items.swing_menu,
        ],
    );

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
