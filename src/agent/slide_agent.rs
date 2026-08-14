use crate::dsl::Slide;
use super::AgentContext;

pub struct SlideAgent;

impl SlideAgent {
    pub fn generate(&self, _ctx: &AgentContext) -> Vec<Slide> {
        // The LLM implementation will replace this placeholder.
        Vec::new()
    }
}
