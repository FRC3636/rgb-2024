use shark::point::{primitives::line, Point};
use ws2812_spi::Ws2812;

use crate::spi::SpiDevice;

pub fn intake_indicator() -> impl Iterator<Item = Point> + Clone {
    line(
        Point::new(0.0, 0.0, 0.0),
        Point::new(0.2286, 0.0, 0.0),
        33,
    )
}

macro_rules! gpio_strip {
    ($file:expr => $fun:ident) => {
        pub fn $fun() -> std::io::Result<Ws2812<SpiDevice>> {
            let dev = SpiDevice::open($file)?;
            Ok(Ws2812::new(dev))
        }
    };
}
gpio_strip!("/dev/spidev0.0" => gpio_10);
gpio_strip!("/dev/spidev1.0" => gpio_18);