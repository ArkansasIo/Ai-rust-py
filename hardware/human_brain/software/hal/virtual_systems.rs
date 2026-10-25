//! Virtual thermal, power, watchdog, and high-speed Neuro-Link models.

#[derive(Debug, Clone)]
pub struct VirtualThermal { pub temperature_c: f32, pub shutdown_c: f32 }
impl VirtualThermal { pub fn new() -> Self { Self { temperature_c: 35.0, shutdown_c: 85.0 } } pub fn step(&mut self, load: f32) { self.temperature_c += load.clamp(0.0, 1.0) * 0.25 - 0.05; self.temperature_c = self.temperature_c.max(20.0); } pub fn emergency(&self) -> bool { self.temperature_c >= self.shutdown_c } }

#[derive(Debug, Clone)]
pub struct VirtualPower { pub input_v: f32, pub current_a: f32 }
impl Default for VirtualPower { fn default() -> Self { Self { input_v: 48.0, current_a: 0.0 } } }

#[derive(Debug, Clone)]
pub struct VirtualWatchdog { pub timeout_ticks: u64, pub last_kick: u64 }
impl VirtualWatchdog { pub fn new(timeout_ticks: u64) -> Self { Self { timeout_ticks, last_kick: 0 } } pub fn kick(&mut self, tick: u64) { self.last_kick = tick; } pub fn expired(&self, tick: u64) -> bool { tick.saturating_sub(self.last_kick) > self.timeout_ticks } }

#[derive(Debug, Clone)]
pub struct VirtualNeuroLink { pub lane_count: u8, pub frames_tx: u64, pub frames_rx: u64 }
impl VirtualNeuroLink { pub fn new(lanes: u8) -> Self { Self { lane_count: lanes.max(1), frames_tx: 0, frames_rx: 0 } } pub fn transmit(&mut self) { self.frames_tx += 1; } pub fn receive(&mut self) { self.frames_rx += 1; } }
