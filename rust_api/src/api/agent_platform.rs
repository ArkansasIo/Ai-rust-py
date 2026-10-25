use axum::{extract::State, Json};
use serde_json::{json, Value};
use std::sync::Arc;
use crate::{agents::AgentRegistry, tools::ToolRegistry, AppState};

pub async fn list_agents() -> Json<Value> {
    let registry = AgentRegistry::new();
    Json(json!({"agents": registry.names()}))
}

pub async fn list_tools() -> Json<Value> {
    let registry = ToolRegistry::new();
    Json(json!({"tools": registry.list()}))
}

pub async fn policy(State(_s): State<Arc<AppState>>) -> Json<Value> {
    Json(json!({"shell":false,"network":false,"writes_require_approval":true,"max_tool_calls":32}))
}
