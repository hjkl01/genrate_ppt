use anyhow::Result;
use crate::llm;

pub async fn generate_outline(topic: String) -> Result<String> {
    llm::chat(
        "You are a professional presentation architect. Generate concise PPT outlines.",
        &format!("Create a slide outline for: {}", topic),
    ).await
}
