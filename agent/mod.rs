pub mod orchestrator;

#[derive(Debug, Clone)]
pub struct AgentContext {
    pub topic: String,
    pub feedback: Vec<String>,
}
