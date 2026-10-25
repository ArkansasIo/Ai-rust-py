//! Virtual low-speed buses and sensor/actuator endpoints.

#[derive(Debug, Clone, Copy, Default)]
pub struct VirtualGpio { pub pins: [bool; 32] }
impl VirtualGpio { pub fn set(&mut self, pin: usize, value: bool) { if pin < self.pins.len() { self.pins[pin] = value; } } }

#[derive(Debug, Clone, Default)]
pub struct VirtualI2c { pub transactions: u64 }
impl VirtualI2c { pub fn transfer(&mut self) { self.transactions += 1; } }

#[derive(Debug, Clone, Default)]
pub struct VirtualSpi { pub transactions: u64 }
impl VirtualSpi { pub fn transfer(&mut self) { self.transactions += 1; } }

#[derive(Debug, Clone, Default)]
pub struct VirtualUart { pub tx_bytes: u64, pub rx_bytes: u64 }
impl VirtualUart { pub fn transmit(&mut self, bytes: usize) { self.tx_bytes += bytes as u64; } pub fn receive(&mut self, bytes: usize) { self.rx_bytes += bytes as u64; } }
