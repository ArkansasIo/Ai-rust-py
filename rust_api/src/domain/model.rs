use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelProvider { Ollama, OpenAICompatible, Local, Custom }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelTask { Chat, Completion, Embedding, Vision, Classification, Reranking }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelSpec { pub name: String, pub provider: ModelProvider, pub tasks: Vec<ModelTask>, pub context_window: u32, pub enabled: bool }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationConfig { pub temperature: f32, pub top_p: f32, pub max_tokens: u32, pub stop: Vec<String> }
