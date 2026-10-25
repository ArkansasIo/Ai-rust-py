//! Hardware-neutral Neuro-Link transport frame and routing contract.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalType { Neural, Sensory, Motor, Learning, Control, Telemetry }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Priority { Critical, High, Normal, Low }

#[derive(Debug, Clone, Copy)]
pub struct NeuroLinkFrame<'a> {
    pub source: u16,
    pub destination: u16,
    pub signal: SignalType,
    pub priority: Priority,
    pub activation: f32,
    pub timestamp_ns: u64,
    pub payload: &'a [u8],
}

pub trait NeuroLinkTransport {
    fn send(&mut self, frame: NeuroLinkFrame<'_>) -> Result<(), &'static str>;
    fn poll(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str>;
}
