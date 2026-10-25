//! Virtual persistent storage model.

#[derive(Debug, Clone)]
pub struct VirtualStorage { pub capacity_bytes: u64, pub used_bytes: u64 }
impl VirtualStorage {
    pub fn new(capacity_bytes: u64) -> Self { Self { capacity_bytes, used_bytes: 0 } }
    pub fn write(&mut self, bytes: u64) -> bool {
        if self.used_bytes.saturating_add(bytes) > self.capacity_bytes { return false; }
        self.used_bytes += bytes; true
    }
}
