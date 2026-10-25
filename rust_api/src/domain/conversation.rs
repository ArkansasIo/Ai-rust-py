use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation { pub id: Uuid, pub user_id: Option<Uuid>, pub title: String, pub system_prompt: Option<String> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationContext { pub conversation_id: Uuid, pub summary: Option<String>, pub token_budget: u32, pub retrieved_memory_ids: Vec<Uuid> }
