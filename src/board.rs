use embassy_rp::{
    Peri,
    gpio::{AnyPin, Input, Level, Output, Pull},
    peripherals::USB,
    pio::Pio,
    pio_programs::rotary_encoder::{PioEncoder, PioEncoderProgram},
    spi::{Config as SpiConfig, Spi},
    usb,
};

use crate::{
    Irqs, KeyGrid, Peripherals, display::Display, key_leds::KeyLeds, rotary_encoder::RotaryEncoder,
};

pub struct Board {
    pub keys: KeyGrid<Input<'static>>,
    pub key_leds: KeyLeds,
    pub display: Display,
    pub rotary_button: Input<'static>,
    pub rotary_encoder: RotaryEncoder,
    pub status_led: Output<'static>,
    pub usb: usb::Driver<'static, USB>,
}

impl Board {
    pub fn new(p: Peripherals) -> Self {
        let mut led_spi_config = SpiConfig::default();
        led_spi_config.frequency = 3_000_000;
        let led_spi =
            Spi::new_blocking_txonly_nosck(p.key_leds_spi, p.key_leds_mosi, led_spi_config);
        let key_leds = KeyLeds::new(led_spi);

        let mut display_spi_config = SpiConfig::default();
        display_spi_config.frequency = 8_000_000;
        let display_spi = Spi::new_blocking(
            p.display_spi,
            p.display_sck,
            p.display_mosi,
            p.display_miso,
            display_spi_config,
        );
        let dc = Output::new(p.display_dc, Level::Low);
        let cs = Output::new(p.display_cs, Level::Low);
        let rst = Output::new(p.display_rst, Level::Low);
        let display = Display::new(display_spi, dc, cs, rst);

        let rotary_button = Input::new(p.rotary_button, Pull::Up);
        let Pio {
            mut common, sm0, ..
        } = Pio::new(p.rotary_encoder_pio, Irqs);
        let prg = PioEncoderProgram::new(&mut common);
        let pio_encoder = PioEncoder::new(
            &mut common,
            sm0,
            p.rotary_encoder_a,
            p.rotary_encoder_b,
            &prg,
        );
        let rotary_encoder = RotaryEncoder::new(pio_encoder);

        let keys: KeyGrid<Peri<'static, AnyPin>> = [
            [p.key_1.into(), p.key_2.into(), p.key_3.into()],
            [p.key_4.into(), p.key_5.into(), p.key_6.into()],
            [p.key_7.into(), p.key_8.into(), p.key_9.into()],
            [p.key_10.into(), p.key_11.into(), p.key_12.into()],
        ];
        let keys = keys.map(|row| row.map(|pin| Input::new(pin, Pull::Up)));

        let status_led = Output::new(p.status_led, Level::Low);

        let usb = usb::Driver::new(p.usb, Irqs);

        Self {
            keys,
            key_leds,
            display,
            rotary_button,
            rotary_encoder,
            status_led,
            usb,
        }
    }
}
