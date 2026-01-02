use embassy_rp::{
    gpio::Output,
    peripherals::SPI1,
    spi::{Blocking, Config, Spi},
};
use sh1106::{mode::GraphicsMode, prelude::SpiInterface};

type SpiOut = Spi<'static, SPI1, Blocking>;
pub struct Display {
    pub display: GraphicsMode<SpiInterface<SpiOut, Output<'static>, Output<'static>>>,
    rst: Output<'static>,
}

impl Display {
    pub fn new(
        spi: SpiOut,
        dc: Output<'static>,
        cs: Output<'static>,
        rst: Output<'static>,
    ) -> Self {
        let mut display_spi_config = Config::default();
        display_spi_config.frequency = 8_000_000;
        let display: GraphicsMode<_> = sh1106::Builder::new().connect_spi(spi, dc, cs).into();
        Self { display, rst }
    }

    pub fn init(&mut self) {
        self.rst.set_high();
        self.display.init().unwrap();
        self.display.flush().unwrap();
    }

    pub fn flush(&mut self) {
        self.display.flush().unwrap();
    }

    pub fn clear(&mut self) {
        self.display.clear();
    }
}
