//! Deterministic scheduler, inter-region messaging, health telemetry, and fault control.

use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct BrainMessage {
    pub timestamp: u64,
    pub source: u16,
    pub destination: u16,
    pub channel: u16,
    pub priority: u8,
    pub payload: Vec<u8>,
}

#[derive(Debug, Default)]
pub struct MessageBus {
    queue: VecDeque<BrainMessage>,
}

impl MessageBus {
    pub fn send(&mut self, message: BrainMessage) { self.queue.push_back(message); }
    pub fn poll(&mut self) -> Option<BrainMessage> { self.queue.pop_front() }
    pub fn len(&self) -> usize { self.queue.len() }
}

#[derive(Debug, Clone, Copy)]
pub struct Tick {
    pub index: u64,
    pub dt_seconds: f32,
}

#[derive(Debug, Clone)]
pub struct Scheduler {
    pub tick_hz: u32,
    pub index: u64,
}

impl Scheduler {
    pub fn new(tick_hz: u32) -> Self { Self { tick_hz: tick_hz.max(1), index: 0 } }
    pub fn step(&mut self) -> Tick {
        let tick = Tick { index: self.index, dt_seconds: 1.0 / self.tick_hz as f32 };
        self.index += 1;
        tick
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthState { Nominal, Warning, Critical, Shutdown }

#[derive(Debug, Clone, Copy)]
pub struct HealthTelemetry {
    pub tick: u64,
    pub temperature_c: f32,
    pub power_v: f32,
    pub watchdog_expired: bool,
    pub thermal_emergency: bool,
    pub state: HealthState,
}

impl HealthTelemetry {
    pub fn evaluate(tick: u64, temperature_c: f32, power_v: f32, watchdog_expired: bool, thermal_emergency: bool) -> Self {
        let state = if thermal_emergency { HealthState::Shutdown }
            else if watchdog_expired { HealthState::Critical }
            else if temperature_c >= 75.0 || power_v < 44.0 { HealthState::Warning }
            else { HealthState::Nominal };
        Self { tick, temperature_c, power_v, watchdog_expired, thermal_emergency, state }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaultKind { CpuStall, ThermalSpike, PowerDrop, NeuroLinkLoss, SensorFailure }

#[derive(Debug, Clone, Copy)]
pub struct FaultInjection {
    pub kind: FaultKind,
    pub active: bool,
    pub severity: u8,
}

impl FaultInjection {
    pub fn new(kind: FaultKind, severity: u8) -> Self { Self { kind, active: true, severity: severity.min(100) } }
    pub fn clear(&mut self) { self.active = false; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scheduler_is_deterministic() {
        let mut s = Scheduler::new(100);
        assert_eq!(s.step().index, 0);
        assert_eq!(s.step().index, 1);
        assert!((s.step().dt_seconds - 0.01).abs() < f32::EPSILON);
    }

    #[test]
    fn health_state_escalates() {
        assert_eq!(HealthTelemetry::evaluate(1, 40.0, 48.0, false, false).state, HealthState::Nominal);
        assert_eq!(HealthTelemetry::evaluate(2, 80.0, 48.0, false, false).state, HealthState::Warning);
        assert_eq!(HealthTelemetry::evaluate(3, 80.0, 48.0, true, false).state, HealthState::Critical);
        assert_eq!(HealthTelemetry::evaluate(4, 90.0, 48.0, true, true).state, HealthState::Shutdown);
    }

    #[test]
    fn fault_injection_can_be_cleared() {
        let mut fault = FaultInjection::new(FaultKind::NeuroLinkLoss, 120);
        assert!(fault.active);
        assert_eq!(fault.severity, 100);
        fault.clear();
        assert!(!fault.active);
    }
}
