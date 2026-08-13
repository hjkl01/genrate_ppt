use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RepairRequest {
    pub slide_id: String,
    pub issues: Vec<String>,
}

pub fn api_routes() -> Vec<&'static str> {
    vec![
        "/api/slides",
        "/api/render",
        "/api/vision/check",
        "/api/repair",
    ]
}
