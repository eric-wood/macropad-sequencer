#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::{
    Peri, bind_interrupts,
    peripherals::{PIO0, USB},
    pio::InterruptHandler as PioInterruptHandler,
    usb::InterruptHandler as UsbInterruptHandler,
};
use embassy_time::Timer;
mod tasks;
use tasks::{read_controls, read_key};
mod display;
use crate::{
    board::Board,
    tasks::{drive_display, read_button, read_rotary_encoder, sequencer, update_lights, usb_midi},
};

use {defmt_rtt as _, panic_probe as _};
mod board;
mod debounced_button;
mod key_leds;
mod macros;
mod menus;
mod rotary_encoder;
mod sequencer_timer;
mod toggle_with_hold;

static COLS: usize = 3;
static ROWS: usize = 4;
static NUM_KEYS: usize = ROWS * COLS;
type KeyGrid<T> = [[T; COLS]; ROWS];

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => UsbInterruptHandler<USB>;
    PIO0_IRQ_0 => PioInterruptHandler<PIO0>;
});

declare_peripherals!(struct Peripherals {
    key_1              => PIN_1,
    key_2              => PIN_2,
    key_3              => PIN_3,
    key_4              => PIN_4,
    key_5              => PIN_5,
    key_6              => PIN_6,
    key_7              => PIN_7,
    key_8              => PIN_8,
    key_9              => PIN_9,
    key_10             => PIN_10,
    key_11             => PIN_11,
    key_12             => PIN_12,
    rotary_encoder_a   => PIN_17,
    rotary_encoder_b   => PIN_18,
    rotary_encoder_pio => PIO0,
    key_leds_spi       => SPI0,
    key_leds_mosi      => PIN_19,
    rotary_button      => PIN_0,
    display_spi        => SPI1,
    display_cs         => PIN_22,
    display_rst        => PIN_23,
    display_dc         => PIN_24,
    display_sck        => PIN_26,
    display_mosi       => PIN_27,
    display_miso       => PIN_28,
    status_led         => PIN_13,
    usb                => USB,
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let peripherals = Peripherals::new(p);

    let board = Board::new(peripherals);

    spawner.spawn(read_controls().unwrap());
    spawner.spawn(update_lights(board.key_leds).unwrap());
    spawner.spawn(read_button(board.rotary_button).unwrap());
    spawner.spawn(read_rotary_encoder(board.rotary_encoder).unwrap());
    spawner.spawn(sequencer().unwrap());
    spawner.spawn(drive_display(board.display).unwrap());
    spawner.spawn(usb_midi(board.usb).unwrap());

    for (y, row) in board.keys.into_iter().enumerate() {
        for (x, input) in row.into_iter().enumerate() {
            spawner.spawn(read_key(input, (x as u8, y as u8)).unwrap());
        }
    }

    let mut status_led = board.status_led;

    loop {
        Timer::after_millis(500).await;
        status_led.toggle();
    }
}
