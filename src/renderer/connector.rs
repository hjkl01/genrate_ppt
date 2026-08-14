use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connector {
    pub from: String,
    pub to: String,
    pub arrow: bool,
}
