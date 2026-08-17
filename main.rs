mod agent;
mod dsl;
mod layout;
mod llm;
mod planner;

use agent::orchestrator::{run, PipelineRequest};
use axum::{routing::{get, post}, Json, Router};
use dsl::GenerateRequest;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/api/generate", post(generate));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn generate(Json(req): Json<GenerateRequest>) -> Json<serde_json::Value> {
    let request = PipelineRequest {
        topic: req.topic,
        audience: None,
        slide_count: 8,
        repair_rounds: 1,
    };

    match run(request).await {
        Ok(result) => Json(serde_json::to_value(result).unwrap_or_else(|_| serde_json::json!({"error":"serialization failed"}))),
        Err(error) => Json(serde_json::json!({"error": error.to_string()})),
    }
}
