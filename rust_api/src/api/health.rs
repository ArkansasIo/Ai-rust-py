use axum::{extract::State, Json};
use serde_json::{json, Value};
use std::sync::Arc;
use crate::AppState;
pub async fn root() -> Json<Value> { Json(json!({"name":"Ai-rust-py","status":"ok"})) }
pub async fn health(State(s): State<Arc<AppState>>) -> Json<Value> { Json(json!({"service":"rust-gateway","status":"ok","ollama":s.config.ollama_url,"python_ml":s.config.python_ml_url})) }
pub async fn info() -> Json<Value> { Json(json!({"version":"0.2.0","architecture":["rust-agent-runtime","ollama","python-ml","mysql","rag","workflow-engine"]})) }
