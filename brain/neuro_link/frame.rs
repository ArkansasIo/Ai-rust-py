use super::{address::NeuroAddress, signal::{NeuroSignal, SignalKind}};

#[derive(Debug, Clone)]
pub struct NeuroLinkFrame {
    pub source: NeuroAddress,
    pub destination: NeuroAddress,
    pub kind: SignalKind,
    pub activation: f32,
    pub learning_signal: f32,
    pub sequence: u64,
    pub timestamp_ms: u64,
    pub ttl: u16,
}

impl NeuroLinkFrame {
    pub fn from_signal(signal: NeuroSignal, sequence: u64) -> Self {
        Self {
            source: signal.source,
            destination: signal.destination,
            kind: signal.kind,
            activation: signal.activation,
            learning_signal: 0.0,
            sequence,
            timestamp_ms: signal.timestamp_ms,
            ttl: 32,
        }
    }
}
