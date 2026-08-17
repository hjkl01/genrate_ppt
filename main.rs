mod agent;
mod dsl;
mod llm;
mod planner;

use agent::orchestrator::{run, PipelineRequest};
use axum::{routing::{get, post}, Json, Router};
use dsl::GenerateRequest;
use serde_json::Value;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[derive(Debug, serde::Deserialize, ToSchema)]
struct HealthResponse {
    status: String,
}

#[derive(OpenApi)]
#[openapi(
    paths(generate, health),
    components(schemas(GenerateRequest, HealthResponse)),
    tags((name = "ppt", description = "AI PPT generation API"))
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/generate", post(generate))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::info!("API listening on http://0.0.0.0:8080");
    tracing::info!("Swagger UI: http://localhost:8080/swagger-ui");
    axum::serve(listener, app).await.unwrap();
}

#[utoipa::path(
    get,
    path = "/health",
    responses((status = 200, description = "Service is healthy", body = HealthResponse))
)]
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok".to_string() })
}

#[utoipa::path(
    post,
    path = "/api/generate",
    request_body = GenerateRequest,
    responses(
        (status = 200, description = "Generated presentation specification", body = Value),
        (status = 500, description = "Generation failed", body = Value)
    )
)]
async fn generate(Json(req): Json<GenerateRequest>) -> Json<serde_json::Value> {
    let request = PipelineRequest {
        topic: req.topic,
        audience: req.audience,
        style: req.style,
        slide_count: req.slide_count,
        repair_rounds: 1,
    };

    match run(request).await {
        Ok(result) => Json(serde_json::to_value(result).unwrap_or_else(|_| serde_json::json!({"error":"serialization failed"}))),
        Err(error) => Json(serde_json::json!({"error": error.to_string()})),
    }
}
