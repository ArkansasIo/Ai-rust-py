use super::{affect::EmotionalState, cognition::CognitiveState, identity::SelfModel, neural::Neuron};

#[derive(Debug)]
pub struct Brain {
    pub neurons: Vec<Neuron>,
    pub cognition: CognitiveState,
    pub emotion: EmotionalState,
    pub self_model: SelfModel,
}

impl Default for Brain {
    fn default() -> Self {
        Self {
            neurons: Vec::new(),
            cognition: CognitiveState::default(),
            emotion: EmotionalState::default(),
            self_model: SelfModel::default(),
        }
    }
}

impl Brain {
    pub fn tick(&mut self, delta_ms: u64) {
        let _ = delta_ms;
        self.emotion.clamp();
        self.cognition.arousal = self.emotion.arousal;
    }
}
