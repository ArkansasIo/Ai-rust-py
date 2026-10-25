use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageRole { System, User, Assistant, Tool }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message { pub id: Uuid, pub conversation_id: Uuid, pub role: MessageRole, pub content: String, pub model: Option<String>, pub tool_call_id: Option<Uuid> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage { pub prompt_tokens: u64, pub completion_tokens: u64, pub total_tokens: u64 }
