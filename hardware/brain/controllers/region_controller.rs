use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrainRegion {
    Brainstem,
    Thalamus,
    Cerebellum,
    Hippocampus,
    Amygdala,
    BasalGanglia,
    FrontalLobe,
    ParietalLobe,
    TemporalLobe,
    OccipitalLobe,
    MotorCortex,
    Somatosensory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControllerState { Booting, Ready, Running, Fault, Shutdown }

#[derive(Debug)]
pub struct RegionController {
    pub region: BrainRegion,
    pub state: ControllerState,
    heartbeat: Instant,
    tick_period: Duration,
}

impl RegionController {
    pub fn new(region: BrainRegion, tick_hz: u32) -> Self {
        let hz = tick_hz.max(1);
        Self { region, state: ControllerState::Booting, heartbeat: Instant::now(), tick_period: Duration::from_micros(1_000_000 / hz as u64) }
    }

    pub fn start(&mut self) { self.state = ControllerState::Ready; }
    pub fn tick(&mut self) { if matches!(self.state, ControllerState::Ready | ControllerState::Running) { self.state = ControllerState::Running; self.heartbeat = Instant::now(); } }
    pub fn heartbeat_age(&self) -> Duration { self.heartbeat.elapsed() }
    pub fn tick_period(&self) -> Duration { self.tick_period }
    pub fn healthy(&self, timeout: Duration) -> bool { self.state != ControllerState::Fault && self.heartbeat_age() <= timeout }
}
