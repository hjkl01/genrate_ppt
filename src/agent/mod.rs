pub mod critic;
pub mod repair;
pub mod slide_agent;

#[derive(Debug, Clone)]
pub struct AgentContext {
    pub topic: String,
    pub feedback: Vec<String>,
}
