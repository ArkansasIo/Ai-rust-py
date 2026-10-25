use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{env, sync::Arc};
use sqlx::{MySqlPool, mysql::MySqlPoolOptions};

#[derive(Clone)]
struct AppState {
    client: Client,
    ml_url: String,
    db: MySqlPool,
}

#[derive(Serialize)]
struct Health {
    service: &'static str,
    status: &'static str,
}

#[derive(Deserialize, Serialize)]
struct PredictRequest {
    model: String,
    input: Value,
}

async fn health() -> impl IntoResponse {
    Json(Health { service: "rust-api", status: "ok" })
}

async fn predict(
    State(state): State<Arc<AppState>>,
    Json(req): Json<PredictRequest>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let url = format!("{}/predict", state.ml_url.trim_end_matches('/'));
    let response = state.client.post(url).json(&req).send().await
        .map_err(|e| (StatusCode::BAD_GATEWAY, e.to_string()))?;

    let status = response.status();
    let body: Value = response.json().await
        .map_err(|e| (StatusCode::BAD_GATEWAY, e.to_string()))?;

    if !status.is_success() {
        return Err((StatusCode::BAD_GATEWAY, body.to_string()));
    }

    sqlx::query("INSERT INTO ai_events (event_type, payload_json) VALUES (?, ?)")
        .bind("prediction")
        .bind(sqlx::types::Json(&body))
        .execute(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(body))
}

async fn info() -> impl IntoResponse {
    Json(json!({
        "name": "ai-rust-py",
        "version": "0.1.0",
        "components": ["rust-api", "python-ml", "mysql"]
    }))
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://ai_user:ai_password@127.0.0.1:3306/ai_ml".into());
    let ml_url = env::var("PYTHON_ML_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:8001".into());
    let host = env::var("RUST_HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port = env::var("RUST_PORT").unwrap_or_else(|_| "8080".into());

    let db = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("database connection failed");

    let state = Arc::new(AppState { client: Client::new(), ml_url, db });

    let app = Router::new()
        .route("/health", get(health))
        .route("/info", get(info))
        .route("/predict", post(predict))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}"))
        .await
        .expect("bind failed");

    println!("Rust AI API listening on http://{host}:{port}");
    axum::serve(listener, app).await.expect("server failed");
}
