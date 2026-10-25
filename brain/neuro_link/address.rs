//! Addressing for nodes in the Neuro-Link fabric.
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NeuroAddress {
    pub brain_id: u16,
    pub region_id: u16,
    pub layer_id: u16,
    pub node_id: u64,
}

impl NeuroAddress {
    pub const fn new(brain_id: u16, region_id: u16, layer_id: u16, node_id: u64) -> Self {
        Self { brain_id, region_id, layer_id, node_id }
    }
}

impl fmt::Display for NeuroAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NLP://{:04x}.{:04x}.{:04x}.{:016x}", self.brain_id, self.region_id, self.layer_id, self.node_id)
    }
}
