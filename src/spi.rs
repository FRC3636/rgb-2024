use std::path::Path;

pub struct SpiDevice {
    spi: spidev::Spidev,

    tx: Option<u8>,
}
impl SpiDevice {
    pub fn open(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let mut spi = spidev::Spidev::open(path)?;
        let options = spidev::SpidevOptions::new()
            .bits_per_word(8)
            .max_speed_hz(3_800_000) // 3.8 MHz
            .build();
        spi.configure(&options)?;
        Ok(Self { spi, tx: None})
    }
}
impl embedded_hal::spi::FullDuplex<u8> for SpiDevice {
    type Error = std::io::Error;
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        let tx_buf = &[self.tx.take().unwrap_or(0)];
        let mut rx_buf = [0];
        let mut transfer = spidev::SpidevTransfer::read_write(tx_buf, &mut rx_buf);
        self.spi.transfer(&mut transfer).map_err(|e| nb::Error::Other(e))?;
        Ok(rx_buf[0])
    }
    fn send(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
        self.tx = Some(byte);
        Ok(())
    }
}