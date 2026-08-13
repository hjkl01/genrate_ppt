use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlideDeck {
    pub slides: Vec<Slide>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Slide {
    pub id: String,
    pub elements: Vec<SlideElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlideElement {
    Text { x: f32, y: f32, text: String },
    Image { x: f32, y: f32, url: String },
    Chart { x: f32, y: f32, spec: String },
}
