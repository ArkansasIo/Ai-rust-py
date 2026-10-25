//! Hardware-independent board interface used by every neural region controller.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bus { I2c, Spi, Uart, Usb, Ethernet }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoardStatus { Offline, Booting, Online, Fault }

pub trait NeuralBoard {
    fn region_id(&self) -> u16;
    fn bus(&self) -> Bus;
    fn status(&self) -> BoardStatus;
    fn send_neuro_link(&mut self, frame: &[u8]) -> Result<(), &'static str>;
    fn receive_neuro_link(&mut self) -> Option<Vec<u8>>;
    fn reset(&mut self);
}
