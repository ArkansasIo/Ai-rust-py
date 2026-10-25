//! High-level cognitive state and processing primitives.

#[derive(Debug, Clone, Default)]
pub struct CognitiveState {
    pub attention: f32,
    pub arousal: f32,
    pub working_memory_load: f32,
    pub confidence: f32,
}

#[derive(Debug, Clone)]
pub struct Goal {
    pub id: String,
    pub description: String,
    pub priority: f32,
}

#[derive(Debug, Clone)]
pub struct Decision {
    pub action: String,
    pub confidence: f32,
    pub rationale: String,
}

#[derive(Debug, Default)]
pub struct WorkingMemory {
    capacity: usize,
    items: Vec<String>,
}

impl WorkingMemory {
    pub fn new(capacity: usize) -> Self {
        Self { capacity, items: Vec::new() }
    }

    pub fn push(&mut self, item: impl Into<String>) {
        if self.items.len() >= self.capacity {
            self.items.remove(0);
        }
        self.items.push(item.into());
    }

    pub fn items(&self) -> &[String] {
        &self.items
    }
}
