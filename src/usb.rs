//use embassy_rp::{peripherals::USB, usb::Driver};
//use embassy_usb::{Config};
//
//type UsbDriver = Driver<'static, USB>;
//
//pub struct Usb<'a> {
//    config: Config<'a>,
//    config_descriptor: [u8; 256],
//    bos_descriptor: [u8; 256]
//    control_buf: [u8; 64],
//}
//
//impl<'a> Usb<'a> {
//    pub fn new() -> Self {
//        let mut config = embassy_usb::Config::new(0xc0de, 0xcafe);
//        config.manufacturer = Some("Heuristic Industries");
//        config.product = Some("Macropad");
//        config.serial_number = Some("123456789");
//        config.max_power = 100;
//        config.max_packet_size_0 = 64;
//
//        let config_descriptor = [0; 256];
//        let bos_descriptor = [0; 256];
//        let control_buf = [0; 64];
//
//        Self { config, config_descriptor, bos_descriptor, control_buf}
//    }
//    pub fn init(&mut self, driver: UsbDriver) {
//        let builder = embassy_usb::Builder::new(
//            driver,
//            self.config,
//            &mut self.config_descriptor,
//            &mut self.bos_descriptor,
//            &mut [], // no msos descriptors
//            &mut self.control_buf,
//        );
//
//        let mut usb = builder.build();
//        usb.run();
//    }
//}
