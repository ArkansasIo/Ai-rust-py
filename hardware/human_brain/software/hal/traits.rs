//! Hardware-independent interfaces used by the brain runtime.

pub trait ComputeBackend {
    fn name(&self) -> &'static str;
    fn tick(&mut self, dt_seconds: f32);
}

pub trait SensorBus {
    fn read(&mut self, channel: u16) -> Result<f32, &'static str>;
}

pub trait ActuatorBus {
    fn write(&mut self, channel: u16, value: f32) -> Result<(), &'static str>;
}

pub trait ThermalMonitor {
    fn temperature_c(&self) -> f32;
    fn emergency_shutdown(&mut self);
}

pub trait NeuralTransport {
    fn send(&mut self, destination: u16, payload: &[u8]) -> Result<(), &'static str>;
    fn poll(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str>;
}

pub trait BrainPlatform {
    type Compute: ComputeBackend;
    type Sensors: SensorBus;
    type Actuators: ActuatorBus;
    type Thermal: ThermalMonitor;
    type Neural: NeuralTransport;

    fn compute(&mut self) -> &mut Self::Compute;
    fn sensors(&mut self) -> &mut Self::Sensors;
    fn actuators(&mut self) -> &mut Self::Actuators;
    fn thermal(&mut self) -> &mut Self::Thermal;
    fn neural(&mut self) -> &mut Self::Neural;
}
