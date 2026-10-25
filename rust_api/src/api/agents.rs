use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use serde_json::{json,Value};
use std::sync::Arc;
use crate::{workflow, AppState};
#[derive(Deserialize)] pub struct AgentRequest { pub task:String, pub agent:Option<String>, pub model:Option<String> }
pub async fn run_agent(State(s):State<Arc<AppState>>,Json(req):Json<AgentRequest>)->Result<Json<Value>,(StatusCode,String)>{
 let agent=req.agent.unwrap_or_else(||"general".into()); let result=workflow::run_simple(&s.http,&s.config.ollama_url,req.model.as_deref().unwrap_or(&s.config.default_model),&agent,&req.task).await.map_err(|e|(StatusCode::BAD_GATEWAY,e.to_string()))?;
 sqlx::query("INSERT INTO ai_events (event_type,payload_json) VALUES (?,?)").bind("agent_run").bind(&result).execute(&s.db).await.map_err(|e|(StatusCode::INTERNAL_SERVER_ERROR,e.to_string()))?;
 Ok(Json(json!({"agent":agent,"result":result})))
}
