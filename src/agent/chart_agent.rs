use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChartSpec {
    pub chart_type: String,
    pub title: String,
    pub data: serde_json::Value,
}

pub async fn create_chart(spec: ChartSpec) -> Result<ChartSpec> {
    Ok(spec)
}
