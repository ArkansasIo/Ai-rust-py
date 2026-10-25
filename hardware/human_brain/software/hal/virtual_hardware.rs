//! Deterministic virtual hardware for development on Raspberry Pi 3 or a PC.

#[derive(Debug, Clone)]
pub struct VirtualHardwareConfig {
    pub sensor_channels: usize,
    pub actuator_channels: usize,
    pub neural_buffer_bytes: usize,
    pub initial_temperature_c: f32,
}

impl Default for VirtualHardwareConfig {
    fn default() -> Self {
        Self {
            sensor_channels: 64,
            actuator_channels: 64,
            neural_buffer_bytes: 64 * 1024,
            initial_temperature_c: 35.0,
        }
    }
}

#[derive(Debug)]
pub struct VirtualHardware {
    sensors: Vec<f32>,
    actuators: Vec<f32>,
    neural_buffer: Vec<u8>,
    temperature_c: f32,
}

impl VirtualHardware {
    pub fn new(config: VirtualHardwareConfig) -> Self {
        Self {
            sensors: vec![0.0; config.sensor_channels],
            actuators: vec![0.0; config.actuator_channels],
            neural_buffer: Vec::with_capacity(config.neural_buffer_bytes),
            temperature_c: config.initial_temperature_c,
        }
    }

    pub fn set_sensor(&mut self, channel: usize, value: f32) {
        if let Some(slot) = self.sensors.get_mut(channel) {
            *slot = value;
        }
    }

    pub fn actuator(&self, channel: usize) -> Option<f32> {
        self.actuators.get(channel).copied()
    }

    pub fn temperature_c(&self) -> f32 {
        self.temperature_c
    }

    pub fn inject_neural_frame(&mut self, payload: &[u8]) {
        self.neural_buffer.extend_from_slice(payload);
    }
}
