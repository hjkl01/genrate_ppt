use anyhow::Result;
use reqwest::Client;

pub struct LlmClient {
    http: Client,
    base_url: String,
    api_key: String,
    model: String,
}

impl LlmClient {
    pub fn from_env() -> Self {
        Self {
            http: Client::new(),
            base_url: std::env::var("OPENAI_BASE_URL")
                .unwrap_or_else(|_| "https://api.openai.com/v1".into()),
            api_key: std::env::var("OPENAI_API_KEY").unwrap_or_default(),
            model: std::env::var("OPENAI_MODEL")
                .unwrap_or_else(|_| "gpt-4.1".into()),
        }
    }

    pub async fn health(&self) -> Result<bool> {
        Ok(!self.base_url.is_empty() && !self.model.is_empty())
    }
}
