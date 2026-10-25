use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;
use crate::{task_queue::{Task, TaskQueue}, AppState};

#[derive(Deserialize)]
pub struct TaskRequest { pub kind: String, pub payload: String }

pub async fn enqueue(State(_state): State<Arc<AppState>>, Json(req): Json<TaskRequest>) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    Ok(Json(json!({"id":Uuid::new_v4().to_string(),"kind":req.kind,"status":"accepted","note":"Use the worker service to consume queued tasks."})))
}

pub async fn queue_status() -> Json<serde_json::Value> {
    Json(json!({"status":"ready","queue":"agent-tasks"}))
}
