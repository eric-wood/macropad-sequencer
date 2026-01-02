use crate::{COLS, ROWS};
use embassy_rp::{
    peripherals::SPI0,
    spi::{Blocking, Spi},
};
use smart_leds::{RGB, SmartLedsWrite};
use ws2812_spi::{Ws2812, devices};

const NUM_LEDS: usize = ROWS * COLS;

pub type Coord = (u8, u8);

type SpiOut = Spi<'static, SPI0, Blocking>;
type Driver = Ws2812<SpiOut, devices::Ws2812>;

pub struct KeyLeds {
    leds: [RGB<u8>; NUM_LEDS],
    driver: Driver,
}

impl KeyLeds {
    pub fn new(spi: SpiOut) -> Self {
        let leds = [RGB::default(); NUM_LEDS];
        let mut driver = Ws2812::new(spi);
        driver.write(leds).unwrap();

        Self { leds, driver }
    }

    pub fn set(&mut self, coord: Coord, value: RGB<u8>) {
        let index = coord.1 as usize * COLS + coord.0 as usize;
        self.leds[index] = value;
    }

    pub fn write(&mut self) {
        self.driver.write(self.leds).unwrap();
    }
}
