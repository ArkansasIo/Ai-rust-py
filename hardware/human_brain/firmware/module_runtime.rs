//! Hardware-neutral runtime contract for a human.brain regional cartridge.
//! Actual silicon drivers are supplied by the selected MCU/SoC HAL.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegionId {
    Brainstem = 0,
    Thalamus = 1,
    Cerebellum = 2,
    Hippocampus = 3,
    Amygdala = 4,
    BasalGanglia = 5,
    Frontal = 6,
    Parietal = 7,
    Temporal = 8,
    Occipital = 9,
    Motor = 10,
    Somatosensory = 11,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleState { Booting, Ready, Running, Fault, Shutdown }

#[derive(Debug, Clone, Copy)]
pub struct ThermalState {
    pub cpu_c: f32,
    pub accelerator_c: f32,
    pub board_c: f32,
    pub inlet_c: f32,
}

pub trait ModuleHal {
    fn set_power_enabled(&mut self, enabled: bool) -> Result<(), &'static str>;
    fn read_thermal(&mut self) -> Result<ThermalState, &'static str>;
    fn set_fan_pwm(&mut self, duty_percent: u8) -> Result<(), &'static str>;
    fn neuro_link_send(&mut self, frame: &[u8]) -> Result<(), &'static str>;
    fn neuro_link_receive(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str>;
    fn watchdog_kick(&mut self) -> Result<(), &'static str>;
}

pub struct RegionRuntime<H: ModuleHal> {
    pub region: RegionId,
    pub state: ModuleState,
    pub thermal: ThermalState,
    pub hal: H,
}

impl<H: ModuleHal> RegionRuntime<H> {
    pub fn new(region: RegionId, hal: H) -> Self {
        Self {
            region,
            state: ModuleState::Booting,
            thermal: ThermalState { cpu_c: 0.0, accelerator_c: 0.0, board_c: 0.0, inlet_c: 0.0 },
            hal,
        }
    }

    pub fn start(&mut self) -> Result<(), &'static str> {
        self.hal.set_power_enabled(true)?;
        self.state = ModuleState::Ready;
        Ok(())
    }

    pub fn tick(&mut self) -> Result<(), &'static str> {
        self.thermal = self.hal.read_thermal()?;
        if self.thermal.cpu_c >= 85.0 || self.thermal.accelerator_c >= 85.0 {
            self.state = ModuleState::Fault;
            self.hal.set_power_enabled(false)?;
            return Err("thermal shutdown");
        }
        let duty = if self.thermal.cpu_c >= 78.0 { 100 } else if self.thermal.cpu_c >= 70.0 { 75 } else { 45 };
        self.hal.set_fan_pwm(duty)?;
        self.hal.watchdog_kick()?;
        self.state = ModuleState::Running;
        Ok(())
    }
}
