//! Deterministic scheduler and inter-region messaging primitives.

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
}
