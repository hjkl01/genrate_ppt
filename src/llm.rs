pub mod provider;

use anyhow::Result;
use serde_json::json;
use provider::LlmProvider;

pub struct OpenAIProvider {
    pub base: String,
    pub key: String,
    pub model: String,
}

impl OpenAIProvider {
    pub fn from_env() -> Self {
        Self {
            base: std::env::var("OPENAI_BASE_URL")
                .unwrap_or_else(|_| "https://api.openai.com/v1".to_string()),
            key: std::env::var("OPENAI_API_KEY").unwrap_or_default(),
            model: std::env::var("OPENAI_MODEL")
                .unwrap_or_else(|_| "gpt-4.1-mini".to_string()),
        }
    }
}

#[async_trait::async_trait]
impl LlmProvider for OpenAIProvider {
    async fn chat(&self, system: &str, prompt: &str) -> Result<String> {
        let client = reqwest::Client::new();
        let resp = client
            .post(format!("{}/chat/completions", self.base.trim_end_matches('/')))
            .bearer_auth(&self.key)
            .json(&json!({
                "model": self.model,
                "messages": [
                    {"role": "system", "content": system},
                    {"role": "user", "content": prompt}
                ]
            }))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        Ok(resp["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or_default()
            .to_string())
    }
}

pub async fn chat(system: &str, prompt: &str) -> Result<String> {
    OpenAIProvider::from_env().chat(system, prompt).await
}
