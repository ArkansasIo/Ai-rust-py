//! Raspberry Pi 3 hybrid hardware adapter boundary.
//!
//! The Pi's CPU remains the physical execution host. Missing accelerators and
//! regional hardware are represented by the portable virtual HAL until native
//! drivers are introduced.

#[derive(Debug, Clone, Copy)]
pub struct RaspberryPi3Platform {
    pub cpu_cores: u8,
    pub memory_mb: u32,
    pub accelerator_emulation: bool,
}

impl Default for RaspberryPi3Platform {
    fn default() -> Self {
        Self { cpu_cores: 4, memory_mb: 768, accelerator_emulation: true }
    }
}

impl RaspberryPi3Platform {
    pub fn target_name(&self) -> &'static str { "raspberry-pi3-hybrid" }
    pub fn supports_accelerator_emulation(&self) -> bool { self.accelerator_emulation }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_profile_matches_hybrid_target() {
        let platform = RaspberryPi3Platform::default();
        assert_eq!(platform.cpu_cores, 4);
        assert_eq!(platform.memory_mb, 768);
        assert!(platform.supports_accelerator_emulation());
        assert_eq!(platform.target_name(), "raspberry-pi3-hybrid");
    }
}
