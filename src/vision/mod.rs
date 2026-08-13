pub mod analyzer;
pub mod feedback;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QaIssue {
    pub slide: usize,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QaReport {
    pub passed: bool,
    pub issues: Vec<QaIssue>,
}
