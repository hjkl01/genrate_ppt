mod dsl;
mod planner;
mod layout;
mod llm;
mod agent;
mod routes;

use axum::{routing::{get, post}, Json, Router};
use dsl::GenerateRequest;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/api/generate", post(generate))
        .route("/api/ai/chat", post(ai_chat));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn generate(Json(req): Json<GenerateRequest>) -> Json<serde_json::Value> {
    let deck = planner::create_plan(req.topic.clone()).await;
    let outline = agent::outline::generate_outline(req.topic).await.ok();
    Json(serde_json::json!({"deck": deck, "outline": outline}))
}

async fn ai_chat(Json(req): Json<serde_json::Value>) -> Json<serde_json::Value> {
    let prompt = req["prompt"].as_str().unwrap_or_default();
    let answer = llm::chat("You are an AI PPT assistant", prompt).await.unwrap_or_default();
    Json(serde_json::json!({"answer": answer}))
}
