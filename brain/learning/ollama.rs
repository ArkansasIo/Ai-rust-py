use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaLearningRequest {
    pub model: String,
    pub prompt: String,
    pub context: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaLearningResponse {
    pub response: String,
}

/// Adapter contract for connecting Ollama inference to the cognitive learning loop.
/// Network transport stays outside the brain model so it can be replaced or mocked.
pub trait OllamaLearner: Send + Sync {
    fn model_name(&self) -> &str;
}

#[derive(Debug, Clone)]
pub struct OllamaModel {
    pub name: String,
}

impl OllamaModel {
    pub fn new(name: impl Into<String>) -> Self { Self { name: name.into() } }
}

impl OllamaLearner for OllamaModel {
    fn model_name(&self) -> &str { &self.name }
}
