use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolRisk { ReadOnly, Low, Medium, High, Critical }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolKind { Http, Database, FileSystem, Shell, Search, Python, Rust, Custom }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition { pub id: Uuid, pub name: String, pub description: String, pub kind: ToolKind, pub risk: ToolRisk, pub enabled: bool, pub input_schema: serde_json::Value, pub output_schema: serde_json::Value }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall { pub id: Uuid, pub tool_id: Uuid, pub input: serde_json::Value, pub approved: bool }
