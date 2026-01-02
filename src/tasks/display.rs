use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
use embedded_graphics::{
    mono_font::{MonoTextStyleBuilder, ascii::FONT_6X10},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};

use crate::display::Display;

pub static DISPLAY_CHANNEL: Channel<ThreadModeRawMutex, DisplayUpdate, 3> = Channel::new();

pub struct DisplayUpdate {
    pub bpm: u32,
}

#[embassy_executor::task]
pub async fn drive_display(mut display: Display) {
    display.init();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    loop {
        let display_update = DISPLAY_CHANNEL.receive().await;
        display.clear();
        let mut buffer = itoa::Buffer::new();
        Text::with_baseline(
            buffer.format(display_update.bpm),
            Point::zero(),
            text_style,
            Baseline::Top,
        )
        .draw(&mut display.display)
        .unwrap();

        display.flush();
    }
}
