use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use crate::{ollama, AppState};
#[derive(Deserialize)] pub struct ChatRequest { pub message: String, pub model: Option<String>, pub conversation_id: Option<String> }
#[derive(Serialize)] pub struct ChatResponse { pub model: String, pub response: String }
pub async fn chat(State(s): State<Arc<AppState>>, Json(req): Json<ChatRequest>) -> Result<Json<ChatResponse>, (StatusCode,String)> {
 let model=req.model.clone().unwrap_or_else(||s.config.default_model.clone());
 let response=ollama::chat(&s.http,&s.config.ollama_url,&model,&req.message).await.map_err(|e|(StatusCode::BAD_GATEWAY,e.to_string()))?;
 sqlx::query("INSERT INTO ai_events (event_type,payload_json) VALUES (?,?)").bind("chat").bind(serde_json::json!({"model":model,"conversation_id":req.conversation_id,"message":req.message})).execute(&s.db).await.map_err(|e|(StatusCode::INTERNAL_SERVER_ERROR,e.to_string()))?;
 Ok(Json(ChatResponse{model,response}))
}
