#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::{
    Peri, bind_interrupts,
    gpio::{AnyPin, Level, Output},
    peripherals::{PIO0, USB},
    pio::InterruptHandler as PioInterruptHandler,
    usb::InterruptHandler as UsbInterruptHandler,
};
use embassy_time::Timer;
mod tasks;
use tasks::{read_controls, read_key};
mod display;
use crate::{
    board::{Board, Peripherals},
    tasks::{drive_display, read_button, read_rotary_encoder, sequencer, update_lights, usb_midi},
};

use {defmt_rtt as _, panic_probe as _};
mod board;
mod debounced_button;
mod key_leds;
mod menus;
mod rotary_encoder;
mod sequencer_timer;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => UsbInterruptHandler<USB>;
    PIO0_IRQ_0 => PioInterruptHandler<PIO0>;
});

static COLS: usize = 3;
static ROWS: usize = 4;
static NUM_KEYS: usize = ROWS * COLS;
type KeyGrid<T> = [[T; COLS]; ROWS];

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let keys: KeyGrid<Peri<'static, AnyPin>> = [
        [p.PIN_1.into(), p.PIN_2.into(), p.PIN_3.into()],
        [p.PIN_4.into(), p.PIN_5.into(), p.PIN_6.into()],
        [p.PIN_7.into(), p.PIN_8.into(), p.PIN_9.into()],
        [p.PIN_10.into(), p.PIN_11.into(), p.PIN_12.into()],
    ];

    let peripherals = Peripherals {
        keys,
        key_leds_spi: p.SPI0,
        key_leds_mosi: p.PIN_19,
        rotary_button: p.PIN_0,
        rotary_encoder_a: p.PIN_17,
        rotary_encoder_b: p.PIN_18,
        rotary_encoder_pio: p.PIO0,
        display_spi: p.SPI1,
        display_cs: p.PIN_22,
        display_rst: p.PIN_23,
        display_dc: p.PIN_24,
        display_sck: p.PIN_26,
        display_mosi: p.PIN_27,
        display_miso: p.PIN_28,
    };

    let board = Board::new(peripherals);

    spawner.spawn(read_controls()).unwrap();
    spawner.spawn(update_lights(board.key_leds)).unwrap();
    spawner.spawn(read_button(board.rotary_button)).unwrap();
    spawner
        .spawn(read_rotary_encoder(board.rotary_encoder))
        .unwrap();
    spawner.spawn(sequencer()).unwrap();
    spawner.spawn(drive_display(board.display)).unwrap();
    spawner.spawn(usb_midi(p.USB)).unwrap();

    for (y, row) in board.keys.into_iter().enumerate() {
        for (x, input) in row.into_iter().enumerate() {
            spawner.spawn(read_key(input, (x as u8, y as u8))).unwrap();
        }
    }

    let mut led = Output::new(p.PIN_13, Level::Low);

    loop {
        Timer::after_millis(500).await;
        led.toggle();
    }
}
