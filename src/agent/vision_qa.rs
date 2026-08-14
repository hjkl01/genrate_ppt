use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionIssue {
    pub kind: String,
    pub description: String,
    pub severity: String,
}

pub struct VisionQAAgent;

impl VisionQAAgent {
    pub async fn analyze(_image_path: &str) -> Vec<VisionIssue> {
        vec![]
    }
}
