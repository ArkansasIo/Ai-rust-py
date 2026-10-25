//! Regional cartridge lifecycle manager for the 12 functional subsystems.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegionStatus { Disabled, Starting, Ready, Warning, Faulted, Restarting }

#[derive(Debug, Clone, Copy)]
pub struct RegionalCartridge {
    pub id: u8,
    pub name: &'static str,
    pub status: RegionStatus,
    pub restart_count: u32,
    pub health_score: u8,
}

impl RegionalCartridge {
    pub const fn new(id: u8, name: &'static str) -> Self { Self { id, name, status: RegionStatus::Disabled, restart_count: 0, health_score: 100 } }
    pub fn start(&mut self) { self.status = RegionStatus::Ready; }
    pub fn disable(&mut self) { self.status = RegionStatus::Disabled; }
    pub fn fault(&mut self) { self.status = RegionStatus::Faulted; self.health_score = 0; }
    pub fn restart(&mut self) { self.status = RegionStatus::Restarting; self.restart_count = self.restart_count.saturating_add(1); self.health_score = 100; self.status = RegionStatus::Ready; }
}

#[derive(Debug)]
pub struct RegionalManager { pub cartridges: [RegionalCartridge; 12] }

impl Default for RegionalManager {
    fn default() -> Self { Self { cartridges: [
        RegionalCartridge::new(0, "brainstem"), RegionalCartridge::new(1, "thalamus"),
        RegionalCartridge::new(2, "cerebellum"), RegionalCartridge::new(3, "hippocampus"),
        RegionalCartridge::new(4, "amygdala"), RegionalCartridge::new(5, "basal-ganglia"),
        RegionalCartridge::new(6, "frontal"), RegionalCartridge::new(7, "parietal"),
        RegionalCartridge::new(8, "temporal"), RegionalCartridge::new(9, "occipital"),
        RegionalCartridge::new(10, "motor"), RegionalCartridge::new(11, "somatosensory"),
    ] } }
}

impl RegionalManager {
    pub fn start_all(&mut self) { for cartridge in &mut self.cartridges { cartridge.start(); } }
    pub fn stop_all(&mut self) { for cartridge in &mut self.cartridges { cartridge.disable(); } }
    pub fn fault(&mut self, id: u8) -> bool { if let Some(c) = self.cartridges.iter_mut().find(|c| c.id == id) { c.fault(); true } else { false } }
    pub fn restart(&mut self, id: u8) -> bool { if let Some(c) = self.cartridges.iter_mut().find(|c| c.id == id) { c.restart(); true } else { false } }
    pub fn ready_count(&self) -> usize { self.cartridges.iter().filter(|c| c.status == RegionStatus::Ready).count() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_regions_start() { let mut manager = RegionalManager::default(); manager.start_all(); assert_eq!(manager.ready_count(), 12); }
    #[test]
    fn fault_and_restart_region() { let mut manager = RegionalManager::default(); manager.start_all(); assert!(manager.fault(6)); assert_eq!(manager.ready_count(), 11); assert!(manager.restart(6)); assert_eq!(manager.ready_count(), 12); assert_eq!(manager.cartridges[6].restart_count, 1); }
}
