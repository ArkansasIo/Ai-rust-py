use axum::{extract::{Path, State}, http::StatusCode, Json};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;
use crate::AppState;

#[derive(Deserialize)]
pub struct TaskRequest { pub kind: String, pub payload: String, pub agent: Option<String>, pub model: Option<String> }

pub async fn enqueue(State(state): State<Arc<AppState>>, Json(req): Json<TaskRequest>) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let id = Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO agent_tasks (id, task, agent, model, status) VALUES (?, ?, ?, ?, 'queued')")
        .bind(&id).bind(&req.payload).bind(req.agent.as_deref().unwrap_or(&req.kind))
        .bind(req.model.as_deref().unwrap_or(&state.config.default_model)).execute(&state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(json!({"id":id,"kind":req.kind,"status":"queued"})))
}

pub async fn get_task(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let row = sqlx::query_as::<_, (String, String, String, String)>("SELECT id, task, status, agent FROM agent_tasks WHERE id = ?")
        .bind(&id).fetch_optional(&state.db).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    match row {
        Some((id, task, status, agent)) => Ok(Json(json!({"id":id,"task":task,"status":status,"agent":agent}))),
        None => Err((StatusCode::NOT_FOUND, "task not found".into())),
    }
}

pub async fn queue_status() -> Json<serde_json::Value> { Json(json!({"status":"ready","queue":"agent-tasks"})) }
