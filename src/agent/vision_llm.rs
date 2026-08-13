use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VisionRequest {
    pub image_base64: String,
    pub prompt: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VisionResult {
    pub issues: Vec<String>,
}

pub async fn analyze(req: VisionRequest) -> VisionResult {
    // OpenAI-compatible vision endpoint integration placeholder.
    // Future: POST /chat/completions with image_url/base64 payload.
    VisionResult {
        issues: vec![format!("vision analysis queued: {}", req.prompt)],
    }
}
