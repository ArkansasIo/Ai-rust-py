//! Learning subsystem for the synthetic-brain runtime.
//!
//! The implementation intentionally separates inference from learning. An
//! Ollama model may propose an answer, while this subsystem records feedback,
//! updates a bounded cognitive state, and produces training/adaptation events.

#[derive(Debug, Clone)]
pub struct LearningEvent {
    pub source: String,
    pub input: String,
    pub output: String,
    pub reward: f32,
    pub confidence: f32,
}

#[derive(Debug, Default)]
pub struct LearningController {
    events: Vec<LearningEvent>,
    pub learning_rate: f32,
}

impl LearningController {
    pub fn new(learning_rate: f32) -> Self {
        Self { events: Vec::new(), learning_rate }
    }

    pub fn observe(&mut self, event: LearningEvent) {
        self.events.push(event);
        if self.events.len() > 10_000 {
            self.events.drain(0..1_000);
        }
    }

    pub fn events(&self) -> &[LearningEvent] { &self.events }

    pub fn mean_reward(&self) -> f32 {
        if self.events.is_empty() { return 0.0; }
        self.events.iter().map(|e| e.reward).sum::<f32>() / self.events.len() as f32
    }
}
