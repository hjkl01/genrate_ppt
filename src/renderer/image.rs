use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageResource {
    pub source: String,
    pub alt: Option<String>,
}
