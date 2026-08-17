use anyhow::{Context, Result};
use crate::{dsl::{GenerateRequest, PresentationSpec}, llm::LlmClient};

const SYSTEM: &str = r#"
You are a presentation architect. Generate a semantic slide plan, not PPTX code and never coordinates.
Every slide must have one clear purpose. Prefer concise text and semantic components.
Allowed slide kinds: cover, section, content, comparison, architecture, timeline, summary.
Allowed components: text, card, image, node, connector, timeline_item, metric.
Return ONLY valid JSON matching this shape:
{"title":"string","theme":"string","slides":[{"id":"s1","kind":"cover","title":"string","subtitle":"string|null","components":[]}]}
Never invent statistics. Use image prompts when visuals would materially help.
"#;

pub async fn plan(client: &LlmClient, request: &GenerateRequest, feedback: &[String]) -> Result<PresentationSpec> {
    let user = format!(
        "Topic: {}\nAudience: {}\nStyle: {}\nTarget slides: {}\nPrevious QA feedback: {}\nCreate the complete semantic slide specification.",
        request.topic,
        request.audience.as_deref().unwrap_or("general professional audience"),
        request.style.as_deref().unwrap_or("modern, clean, professional"),
        request.slide_count,
        if feedback.is_empty() { "none".into() } else { feedback.join("; ") },
    );
    let value = client.json(SYSTEM, &user).await?;
    let spec: PresentationSpec = serde_json::from_value(value).context("LLM output does not match PresentationSpec")?;
    spec.validate(request.slide_count)?;
    Ok(spec)
}
