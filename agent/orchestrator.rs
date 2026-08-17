use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::{dsl::{GenerateRequest, PresentationSpec}, llm::LlmClient, planner};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineRequest {
    pub topic: String,
    #[serde(default)]
    pub audience: Option<String>,
    #[serde(default)]
    pub style: Option<String>,
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
    pub spec: PresentationSpec,
    pub feedback: Vec<String>,
}

pub async fn run(request: PipelineRequest) -> Result<PipelineResult> {
    let topic = request.topic.trim().to_string();
    if topic.is_empty() { return Err(anyhow!("topic is required")); }
    if !(1..=100).contains(&request.slide_count) { return Err(anyhow!("slide_count must be between 1 and 100")); }

    let llm = LlmClient::from_env()?;
    let input = GenerateRequest {
        topic: topic.clone(),
        audience: request.audience,
        style: request.style,
        slide_count: request.slide_count,
    };
    let mut feedback = Vec::new();
    let mut spec = None;

    for attempt in 0..=request.repair_rounds {
        match planner::plan(&llm, &input, &feedback).await {
            Ok(candidate) => {
                spec = Some(candidate);
                break;
            }
            Err(error) if attempt < request.repair_rounds => {
                feedback.push(format!("planner attempt {} failed validation: {error}", attempt + 1));
            }
            Err(error) => return Err(error),
        }
    }

    let spec = spec.ok_or_else(|| anyhow!("planner produced no presentation spec"))?;
    feedback.push(format!("semantic DSL validated: {} slides", spec.slides.len()));
    Ok(PipelineResult { stage: "semantic_plan_ready".into(), topic, spec, feedback })
}
