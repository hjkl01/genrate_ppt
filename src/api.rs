use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RenderRequest {
    pub slide_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepairRequest {
    pub issues: Vec<String>,
}

pub fn routes() {
    // /api/slides
    // /api/render
    // /api/vision/check
    // /api/repair
}
