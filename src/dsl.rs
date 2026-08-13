use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateRequest {
    pub topic: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SlideType {
    Cover,
    Section,
    Content,
    Comparison,
    Architecture,
    Timeline,
    Summary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Slide {
    pub slide_type: SlideType,
    pub title: String,
    pub components: Vec<Component>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Component {
    Text(String),
    Image(String),
    Node(String),
    Arrow(String, String),
}
