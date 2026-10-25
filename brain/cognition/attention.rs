#[derive(Debug, Clone)]
pub struct AttentionSignal {
    pub source: String,
    pub salience: f32,
    pub urgency: f32,
    pub novelty: f32,
}

#[derive(Debug, Default)]
pub struct AttentionController;

impl AttentionController {
    pub fn score(signal: &AttentionSignal) -> f32 {
        (signal.salience * 0.5 + signal.urgency * 0.3 + signal.novelty * 0.2).clamp(0.0, 1.0)
    }
}
