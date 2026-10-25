//! Persistent identity and personality representations.

#[derive(Debug, Clone)]
pub struct Personality {
    pub openness: f32,
    pub conscientiousness: f32,
    pub extraversion: f32,
    pub agreeableness: f32,
    pub emotional_stability: f32,
}

impl Default for Personality {
    fn default() -> Self {
        Self {
            openness: 0.5,
            conscientiousness: 0.5,
            extraversion: 0.5,
            agreeableness: 0.5,
            emotional_stability: 0.5,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SelfModel {
    pub identity_name: String,
    pub personality: Personality,
    pub values: Vec<String>,
    pub beliefs: Vec<String>,
    pub capabilities: Vec<String>,
    pub limitations: Vec<String>,
    pub autobiographical_memory_ids: Vec<String>,
}
