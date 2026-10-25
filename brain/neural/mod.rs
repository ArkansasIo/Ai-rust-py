//! Neural primitives used by the synthetic-brain runtime.

#[derive(Debug, Clone, Copy)]
pub struct NeuralSignal {
    pub value: f32,
    pub timestamp_ms: u64,
}

#[derive(Debug, Clone)]
pub struct Neuron {
    pub id: u64,
    pub membrane_potential: f32,
    pub threshold: f32,
    pub refractory_ms: u32,
}

impl Neuron {
    pub fn integrate(&mut self, input: f32) -> bool {
        self.membrane_potential += input;
        if self.membrane_potential >= self.threshold {
            self.membrane_potential = 0.0;
            return true;
        }
        false
    }
}

#[derive(Debug, Clone)]
pub struct Synapse {
    pub source: u64,
    pub target: u64,
    pub weight: f32,
    pub delay_ms: u32,
}

impl Synapse {
    pub fn transmit(&self, signal: NeuralSignal) -> NeuralSignal {
        NeuralSignal {
            value: signal.value * self.weight,
            timestamp_ms: signal.timestamp_ms + self.delay_ms as u64,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Neuromodulator {
    Dopamine,
    Serotonin,
    Norepinephrine,
    Acetylcholine,
    Gaba,
    Glutamate,
    Oxytocin,
    Endorphin,
}
