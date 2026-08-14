use anyhow::Result;

#[async_trait::async_trait]
pub trait LlmProvider: Send + Sync {
    async fn chat(&self, system: &str, prompt: &str) -> Result<String>;

    async fn stream(&self, system: &str, prompt: &str) -> Result<Vec<String>> {
        let value = self.chat(system, prompt).await?;
        Ok(value.chars().map(|c| c.to_string()).collect())
    }
}
