use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use serde_json::Value;
use std::sync::Arc;
use crate::{runtime::AgentRuntime, models::AgentTask, AppState};

#[derive(Deserialize)]
pub struct AgentRequest {
    pub task: String,
    pub agent: Option<String>,
    pub model: Option<String>,
    pub conversation_id: Option<String>,
}

pub async fn run_agent(
    State(s): State<Arc<AppState>>,
    Json(req): Json<AgentRequest>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let request = AgentTask {
        task: req.task,
        agent: req.agent.unwrap_or_else(|| "general".into()),
        model: req.model,
        conversation_id: req.conversation_id,
    };
    let runtime = AgentRuntime::new();
    let result = runtime.run(&s.http, &s.config.ollama_url, &s.config.default_model, request)
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, e.to_string()))?;
    sqlx::query("INSERT INTO ai_events (event_type,payload_json) VALUES (?,?)")
        .bind("agent_run")
        .bind(serde_json::to_value(&result).unwrap_or_else(|_| serde_json::json!({})))
        .execute(&s.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(serde_json::to_value(result).unwrap_or_else(|_| serde_json::json!({"status":"error"}))))
}
