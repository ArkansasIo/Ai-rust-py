use crate::domain::model::{GenerationConfig, ModelSpec};

pub struct ModelService;
impl ModelService {
    pub fn validate(spec: &ModelSpec, generation: &GenerationConfig) -> Result<(), String> {
        if spec.name.trim().is_empty() { return Err("model name is required".into()); }
        if !(0.0..=2.0).contains(&generation.temperature) { return Err("temperature must be between 0 and 2".into()); }
        if !(0.0..=1.0).contains(&generation.top_p) { return Err("top_p must be between 0 and 1".into()); }
        Ok(())
    }
}
