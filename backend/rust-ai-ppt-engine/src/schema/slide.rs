use serde::{Deserialize, Serialize};
use super::element::SlideElement;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Slide {
    pub id: String,
    pub title: String,
    pub elements: Vec<SlideElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlideDeck {
    pub id: String,
    pub slides: Vec<Slide>,
}
