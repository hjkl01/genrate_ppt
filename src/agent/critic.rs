use crate::layout::PositionedSlide;

pub struct CriticAgent;

impl CriticAgent {
    pub fn review(&self, slide: &PositionedSlide) -> Vec<String> {
        let mut issues = Vec::new();

        if slide.components.len() > 8 {
            issues.push("slide contains too many visual components".into());
        }

        issues
    }
}
