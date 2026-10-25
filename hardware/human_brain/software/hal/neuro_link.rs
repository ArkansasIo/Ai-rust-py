//! Deterministic Neuro-Link frame model for inter-regional high-speed transport.
//!
//! This models the protocol boundary; physical signaling, SerDes, and PCB
//! implementation remain hardware-specific.

pub const PROTOCOL_VERSION: u8 = 1;
pub const HEADER_BYTES: usize = 25;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NeuroLinkFrame {
    pub source: u16,
    pub destination: u16,
    pub channel: u16,
    pub priority: u8,
    pub sequence: u32,
    pub timestamp: u64,
    pub payload: Vec<u8>,
}

impl NeuroLinkFrame {
    pub fn new(source: u16, destination: u16, channel: u16, priority: u8, sequence: u32, timestamp: u64, payload: Vec<u8>) -> Self {
        Self { source, destination, channel, priority, sequence, timestamp, payload }
    }

    pub fn checksum(&self) -> u32 {
        let mut hash = 0x811C9DC5u32;
        for byte in self.bytes_without_checksum() {
            hash ^= byte as u32;
            hash = hash.wrapping_mul(0x01000193);
        }
        hash
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut out = self.bytes_without_checksum();
        out.extend_from_slice(&self.checksum().to_le_bytes());
        out
    }

    fn bytes_without_checksum(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(HEADER_BYTES + self.payload.len());
        out.push(PROTOCOL_VERSION);
        out.extend_from_slice(&self.source.to_le_bytes());
        out.extend_from_slice(&self.destination.to_le_bytes());
        out.extend_from_slice(&self.channel.to_le_bytes());
        out.push(self.priority);
        out.extend_from_slice(&self.sequence.to_le_bytes());
        out.extend_from_slice(&self.timestamp.to_le_bytes());
        out.extend_from_slice(&(self.payload.len() as u32).to_le_bytes());
        out.extend_from_slice(&self.payload);
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_encoding_contains_payload_and_checksum() {
        let frame = NeuroLinkFrame::new(1, 2, 3, 4, 5, 6, vec![10, 20, 30]);
        let encoded = frame.encode();
        assert!(encoded.len() > HEADER_BYTES);
        assert_eq!(&encoded[encoded.len() - 4..], &frame.checksum().to_le_bytes());
    }

    #[test]
    fn identical_frames_are_deterministic() {
        let a = NeuroLinkFrame::new(1, 2, 3, 4, 5, 6, vec![7, 8]);
        let b = NeuroLinkFrame::new(1, 2, 3, 4, 5, 6, vec![7, 8]);
        assert_eq!(a.encode(), b.encode());
    }
}
