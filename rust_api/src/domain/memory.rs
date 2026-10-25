use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryType { Semantic, Episodic, Procedural, Preference, Summary }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryRecord { pub id: Uuid, pub user_id: Option<Uuid>, pub memory_type: MemoryType, pub content: String, pub embedding: Option<Vec<f32>>, pub importance: f32, pub metadata: serde_json::Value }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryQuery { pub user_id: Option<Uuid>, pub query: String, pub top_k: usize, pub memory_types: Vec<MemoryType> }
