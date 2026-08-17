use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::agent::{outline, AgentContext};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineRequest {
    pub topic: String,
    #[serde(default)]
    pub audience: Option<String>,
    #[serde(default = "default_slide_count")]
    pub slide_count: usize,
    #[serde(default)]
    pub repair_rounds: usize,
}

fn default_slide_count() -> usize { 8 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineResult {
    pub stage: String,
    pub topic: String,
    pub outline: String,
    pub feedback: Vec<String>,
}

/// Coordinates the presentation workflow without coupling agents to HTTP or PPTX.
/// Each stage can later be replaced by a stronger implementation (structured LLM,
/// renderer, vision model, or repair agent) without changing the API layer.
pub async fn run(request: PipelineRequest) -> Result<PipelineResult> {
    let topic = request.topic.trim().to_string();
    if topic.is_empty() {
        return Err(anyhow!("topic is required"));
    }
    if !(1..=100).contains(&request.slide_count) {
        return Err(anyhow!("slide_count must be between 1 and 100"));
    }

    let mut context = AgentContext { topic: topic.clone(), feedback: Vec::new() };
    let outline_text = outline::generate_outline(topic.clone()).await?;

    // Keep the orchestration state explicit. The next stage consumes this context
    // and produces a semantic slide DSL rather than directly emitting PPTX bytes.
    context.feedback.push(format!("outline generated for {} slides", request.slide_count));
    if let Some(audience) = request.audience.as_deref().filter(|v| !v.trim().is_empty()) {
        context.feedback.push(format!("target audience: {audience}"));
    }

    let stage = if request.repair_rounds > 0 { "planned_with_repair_budget" } else { "planned" };
    Ok(PipelineResult {
        stage: stage.into(),
        topic: context.topic,
        outline: outline_text,
        feedback: context.feedback,
    })
}
