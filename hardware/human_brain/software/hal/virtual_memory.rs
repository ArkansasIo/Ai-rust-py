//! Virtual RAM model.

#[derive(Debug, Clone)]
pub struct VirtualMemory { pub capacity_bytes: usize, pub used_bytes: usize }
impl VirtualMemory {
    pub fn new(capacity_bytes: usize) -> Self { Self { capacity_bytes, used_bytes: 0 } }
    pub fn allocate(&mut self, bytes: usize) -> bool {
        if self.used_bytes.saturating_add(bytes) > self.capacity_bytes { return false; }
        self.used_bytes += bytes; true
    }
    pub fn free(&mut self, bytes: usize) { self.used_bytes = self.used_bytes.saturating_sub(bytes); }
}
