use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskPriority { Low, Normal, High, Critical }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus { Queued, Running, Completed, Failed, Cancelled }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task { pub id: Uuid, pub name: String, pub priority: TaskPriority, pub status: TaskStatus, pub payload: serde_json::Value, pub attempts: u32, pub max_attempts: u32 }
