use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone)]
pub struct LlmClient {
    client: Client,
    base_url: String,
    api_key: String,
    model: String,
}

impl LlmClient {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            client: Client::new(),
            base_url: std::env::var("LLM_BASE_URL").unwrap_or_else(|_| "https://api.openai.com/v1".into()).trim_end_matches('/').into(),
            api_key: std::env::var("LLM_API_KEY").context("LLM_API_KEY is required")?,
            model: std::env::var("LLM_MODEL").unwrap_or_else(|_| "gpt-5.6".into()),
        })
    }

    pub async fn json(&self, system: &str, user: &str) -> Result<Value> {
        let body = ChatRequest {
            model: &self.model,
            messages: vec![Message { role: "system", content: system }, Message { role: "user", content: user }],
            response_format: ResponseFormat { kind: "json_object" },
            temperature: 0.2,
        };
        let response: ChatResponse = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .bearer_auth(&self.api_key)
            .json(&body)
            .send().await?
            .error_for_status()?
            .json().await?;
        let content = response.choices.first().context("LLM returned no choices")?.message.content.trim();
        let content = content.strip_prefix("```json").unwrap_or(content).trim().trim_end_matches("```").trim();
        serde_json::from_str(content).with_context(|| format!("LLM returned invalid JSON: {content}"))
    }
}

#[derive(Serialize)]
struct ChatRequest<'a> { model: &'a str, messages: Vec<Message<'a>>, response_format: ResponseFormat<'a>, temperature: f32 }
#[derive(Serialize)]
struct Message<'a> { role: &'a str, content: &'a str }
#[derive(Serialize)]
struct ResponseFormat<'a> { #[serde(rename = "type")] kind: &'a str }
#[derive(Deserialize)]
struct ChatResponse { choices: Vec<Choice> }
#[derive(Deserialize)]
struct Choice { message: AssistantMessage }
#[derive(Deserialize)]
struct AssistantMessage { content: String }
