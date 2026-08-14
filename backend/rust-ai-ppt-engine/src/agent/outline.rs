use crate::schema::slide::{Slide, SlideDeck};

pub struct OutlineAgent;

impl OutlineAgent {
    pub fn new() -> Self {
        Self
    }

    pub fn create_outline(&self, topic: &str) -> SlideDeck {
        let slides = vec![
            Slide {
                id: "slide-1".to_string(),
                title: format!("{} - Introduction", topic),
                elements: vec![],
            },
            Slide {
                id: "slide-2".to_string(),
                title: "Architecture and Key Ideas".to_string(),
                elements: vec![],
            },
        ];

        SlideDeck {
            id: topic.to_string(),
            slides,
        }
    }
}
