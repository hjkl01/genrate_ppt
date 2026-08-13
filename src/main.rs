mod dsl;
mod planner;
mod layout;

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
    let deck = planner::create_plan(req.topic).await;
    Json(serde_json::json!({"deck": deck}))
}
