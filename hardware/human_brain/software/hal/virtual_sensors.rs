//! Virtual camera, microphone, and proprioceptive sensor sources.

#[derive(Debug, Clone)]
pub struct VirtualCamera { pub width: u32, pub height: u32, pub frames: u64 }
impl VirtualCamera { pub fn new(width: u32, height: u32) -> Self { Self { width, height, frames: 0 } } pub fn capture(&mut self) { self.frames += 1; } }

#[derive(Debug, Clone)]
pub struct VirtualMicrophone { pub sample_rate: u32, pub samples: u64 }
impl VirtualMicrophone { pub fn new(sample_rate: u32) -> Self { Self { sample_rate, samples: 0 } } pub fn capture(&mut self, count: u64) { self.samples += count; } }

#[derive(Debug, Clone, Default)]
pub struct VirtualProprioception { pub joint_channels: Vec<f32>, pub touch_channels: Vec<f32> }
