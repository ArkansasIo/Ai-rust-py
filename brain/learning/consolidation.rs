#[derive(Debug, Clone, Copy)]
pub struct MemoryConsolidator {
    pub threshold: f32,
    pub decay: f32,
}

impl MemoryConsolidator {
    pub fn new(threshold: f32, decay: f32) -> Self {
        Self { threshold, decay }
    }

    pub fn consolidate(&self, strength: f32, importance: f32) -> f32 {
        let reinforced = strength + importance;
        if reinforced >= self.threshold { reinforced } else { reinforced * self.decay }
    }
}
