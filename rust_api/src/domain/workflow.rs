use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowStepKind { Prompt, Model, Tool, Condition, Transform, MemoryRead, MemoryWrite, Python, Rust }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep { pub id: Uuid, pub name: String, pub kind: WorkflowStepKind, pub config: serde_json::Value, pub next: Vec<Uuid> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow { pub id: Uuid, pub name: String, pub version: u32, pub enabled: bool, pub steps: Vec<WorkflowStep> }
