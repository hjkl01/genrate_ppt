use crate::schema::slide::SlideDeck;

pub struct OutlineAgent;

impl OutlineAgent {
    pub fn new() -> Self {
        Self
    }

    pub fn create_outline(&self, topic: &str) -> SlideDeck {
        SlideDeck {
            id: topic.to_string(),
            slides: vec![],
        }
    }
}
