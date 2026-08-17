pub mod chart_agent;
pub mod critic;
pub mod image_agent;
pub mod orchestrator;
pub mod outline;
pub mod repair;
pub mod repair_agent;
pub mod slide_agent;
pub mod vision_qa;

#[derive(Debug, Clone)]
pub struct AgentContext {
    pub topic: String,
    pub feedback: Vec<String>,
}
