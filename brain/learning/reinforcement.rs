#[derive(Debug, Clone, Copy)]
pub struct ReinforcementLearner {
    pub learning_rate: f32,
    pub discount: f32,
}

impl ReinforcementLearner {
    pub fn new(learning_rate: f32, discount: f32) -> Self {
        Self { learning_rate, discount }
    }

    pub fn td_error(&self, reward: f32, value: f32, next_value: f32, terminal: bool) -> f32 {
        let target = if terminal { reward } else { reward + self.discount * next_value };
        target - value
    }

    pub fn update(&self, value: f32, error: f32) -> f32 {
        value + self.learning_rate * error
    }
}
