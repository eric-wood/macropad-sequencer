use embassy_rp::{
    Peri,
    gpio::{Input, Level, Output, Pull},
    peripherals::{PIN_0, PIN_19, PIN_22, PIN_23, PIN_24, PIN_26, PIN_27, PIN_28, SPI0, SPI1},
    spi::{Config as SpiConfig, Spi},
};

use crate::{display::Display, key_leds::KeyLeds};

pub struct Peripherals {
    pub key_leds_spi: Peri<'static, SPI0>,
    pub key_leds_mosi: Peri<'static, PIN_19>,
    pub rotary_button: Peri<'static, PIN_0>,
    pub display_spi: Peri<'static, SPI1>,
    pub display_cs: Peri<'static, PIN_22>,
    pub display_rst: Peri<'static, PIN_23>,
    pub display_dc: Peri<'static, PIN_24>,
    pub display_sck: Peri<'static, PIN_26>,
    pub display_mosi: Peri<'static, PIN_27>,
    pub display_miso: Peri<'static, PIN_28>,
}

pub struct Board {
    pub key_leds: KeyLeds,
    pub display: Display,
    pub rotary_button: Input<'static>,
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

        Self {
            key_leds,
            display,
            rotary_button,
        }
    }
}
