use axum::Json;
use serde::{Deserialize, Serialize};
use crate::agent::outline::OutlineAgent;

#[derive(Debug, Deserialize)]
pub struct GenerateRequest {
    pub topic: String,
}

#[derive(Debug, Serialize)]
pub struct GenerateResponse {
    pub deck: crate::schema::slide::SlideDeck,
}

pub async fn generate(Json(req): Json<GenerateRequest>) -> Json<GenerateResponse> {
    let agent = OutlineAgent::new();
    let deck = agent.create_outline(&req.topic);
    Json(GenerateResponse { deck })
}
