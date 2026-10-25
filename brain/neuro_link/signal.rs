use super::address::NeuroAddress;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalKind {
    Excitatory,
    Inhibitory,
    Modulatory,
    Learning,
    Control,
    Broadcast,
}

#[derive(Debug, Clone, Copy)]
pub struct NeuroSignal {
    pub source: NeuroAddress,
    pub destination: NeuroAddress,
    pub kind: SignalKind,
    pub activation: f32,
    pub timestamp_ms: u64,
}
