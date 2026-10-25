//! Deterministic virtual CPU scheduler.

#[derive(Debug, Clone)]
pub struct VirtualCpu { pub cores: u16, pub instructions: u64 }
impl VirtualCpu {
    pub fn new(cores: u16) -> Self { Self { cores: cores.max(1), instructions: 0 } }
    pub fn execute(&mut self, instructions: u64) { self.instructions = self.instructions.saturating_add(instructions); }
}
