#![no_std]
#![no_main]

use core::sync::atomic::AtomicU32;

use embassy_executor::Spawner;
use embassy_rp::{
    Peri, bind_interrupts,
    gpio::{AnyPin, Level, Output},
    peripherals::{PIO0, USB},
    pio::InterruptHandler as PioInterruptHandler,
    usb::{Driver, Instance, InterruptHandler as UsbInterruptHandler},
};
use embassy_time::{Duration, Timer};
use embassy_usb::{class::midi::MidiClass, driver::EndpointError};
mod tasks;
use tasks::{read_controls, read_key};
mod display;
use crate::{
    board::{Board, Peripherals},
    tasks::{drive_display, read_button, read_rotary_encoder, sequencer, update_lights},
};
use midi_convert::{
    midi_types::{MidiMessage, Note},
    render_slice::MidiRenderSlice,
};
use usbd_midi::{CableNumber, UsbMidiEventPacket};

use {defmt_rtt as _, panic_probe as _};
mod board;
mod debounced_button;
mod key_leds;
mod menus;
mod rotary_encoder;
mod usb;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => UsbInterruptHandler<USB>;
    PIO0_IRQ_0 => PioInterruptHandler<PIO0>;
});

const COLS: usize = 3;
const ROWS: usize = 4;
type KeyGrid<T> = [[T; COLS]; ROWS];

static SPEED_MS: AtomicU32 = AtomicU32::new(120);

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

    for (y, row) in board.keys.into_iter().enumerate() {
        for (x, input) in row.into_iter().enumerate() {
            spawner.spawn(read_key(input, (x as u8, y as u8))).unwrap();
        }
    }

    let mut led = Output::new(p.PIN_13, Level::Low);

    //let driver = embassy_rp::usb::Driver::new(p.USB, Irqs);
    //let mut usb_config = embassy_usb::Config::new(0xc0de, 0xcafe);
    //usb_config.manufacturer = Some("Heuristic Industries");
    //usb_config.product = Some("Macropad");
    //usb_config.serial_number = Some("123456789");
    //usb_config.max_power = 100;
    //usb_config.max_packet_size_0 = 64;

    //let mut config_descriptor = [0; 256];
    //let mut bos_descriptor = [0; 256];
    //let mut control_buf = [0; 64];

    //let mut usb_builder = embassy_usb::Builder::new(
    //    driver,
    //    usb_config,
    //    &mut config_descriptor,
    //    &mut bos_descriptor,
    //    &mut [], // no msos descriptors
    //    &mut control_buf,
    //);

    //let mut class = MidiClass::new(&mut usb_builder, 1, 1, 64);
    //let mut usb = usb_builder.build();
    //let usb_fut = usb.run();

    //let midi_fut = async {
    //    loop {
    //        class.wait_connection().await;
    //        let _ = midi_echo(&mut class, &mut led).await;
    //    }
    //};

    //join(usb_fut, midi_fut).await;
    loop {
        Timer::after_millis(500).await;
        led.toggle();
    }
}

struct Disconnected {}

impl From<EndpointError> for Disconnected {
    fn from(val: EndpointError) -> Self {
        match val {
            EndpointError::BufferOverflow => defmt::panic!("Buffer overflow"),
            EndpointError::Disabled => Disconnected {},
        }
    }
}

async fn midi_echo<'d, T: Instance + 'd>(
    class: &mut MidiClass<'d, Driver<'d, T>>,
    led: &mut Output<'_>,
) -> Result<(), Disconnected> {
    //let mut buf = [0; 64];

    let on_message = MidiMessage::NoteOn(1.into(), Note::C0, 0x34.into());
    let mut on_buffer = [0u8; 3];
    on_message.render_slice(&mut on_buffer);
    let on_packet =
        UsbMidiEventPacket::try_from_payload_bytes(CableNumber::Cable0, &on_buffer).unwrap();

    let off_message = MidiMessage::NoteOff(1.into(), Note::C0, 0x34.into());
    let mut off_buffer = [0u8; 3];
    off_message.render_slice(&mut off_buffer);
    let off_packet =
        UsbMidiEventPacket::try_from_payload_bytes(CableNumber::Cable0, &off_buffer).unwrap();

    loop {
        led.set_high();
        class.write_packet(on_packet.as_raw_bytes()).await?;
        Timer::after(Duration::from_millis(500)).await;

        led.set_low();
        class.write_packet(off_packet.as_raw_bytes()).await?;
        Timer::after(Duration::from_millis(500)).await;

        //let n = class.read_packet(&mut buf).await?;
        //let data = &buf[..n];

        //info!("data: {:x}", data);
        //class.write_packet(data).await?;
    }
}
