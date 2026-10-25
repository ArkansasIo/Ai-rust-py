//! Virtual accelerator model used when physical GPU hardware is absent.

#[derive(Debug, Clone)]
pub struct VirtualGpu { pub units: u16, pub jobs_completed: u64 }
impl VirtualGpu {
    pub fn new(units: u16) -> Self { Self { units: units.max(1), jobs_completed: 0 } }
    pub fn submit(&mut self, jobs: u32) { self.jobs_completed = self.jobs_completed.saturating_add(jobs as u64); }
}
