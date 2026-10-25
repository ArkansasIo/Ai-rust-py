mod api;
mod config;
mod models;
mod ollama;
mod workflow;

use axum::{routing::{get, post}, Router};
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use std::{env, sync::Arc, time::Duration};
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, cors::CorsLayer, timeout::TimeoutLayer, trace::TraceLayer};
use tracing::info;

#[derive(Clone)]
pub struct AppState {
    pub db: MySqlPool,
    pub http: reqwest::Client,
    pub config: config::Config,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt().with_env_filter(env::var("RUST_LOG").unwrap_or_else(|_| "info".into())).init();

    let config = config::Config::from_env();
    let db = MySqlPoolOptions::new().max_connections(config.db_max_connections)
        .connect(&config.database_url).await.expect("database connection failed");
    sqlx::migrate!("../database/migrations").run(&db).await.expect("database migration failed");

    let state = Arc::new(AppState { db, http: reqwest::Client::new(), config: config.clone() });
    let app = Router::new()
        .route("/", get(api::health::root))
        .route("/health", get(api::health::health))
        .route("/info", get(api::health::info))
        .route("/api/v1/chat", post(api::chat::chat))
        .route("/api/v1/agents/run", post(api::agents::run_agent))
        .route("/api/v1/workflows/run", post(api::workflows::run_workflow))
        .route("/api/v1/models", get(api::models::models))
        .route("/api/v1/embeddings", post(api::rag::embed))
        .route("/api/v1/memory/search", post(api::memory::search))
        .with_state(state)
        .layer(ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(CompressionLayer::new())
            .layer(CorsLayer::permissive())
            .layer(TimeoutLayer::new(Duration::from_secs(120))));

    let listener = tokio::net::TcpListener::bind((&config.host[..], config.port)).await.expect("bind failed");
    info!(address = %listener.local_addr().unwrap(), "AI Rust gateway started");
    axum::serve(listener, app).await.expect("server failed");
}
