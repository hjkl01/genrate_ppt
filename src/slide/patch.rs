use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SlidePatch {
    pub operations: Vec<SlideOperation>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlideOperation {
    ReplaceText {
        element_id: String,
        text: String,
    },
    Move {
        element_id: String,
        x: f32,
        y: f32,
    },
    Resize {
        element_id: String,
        width: f32,
        height: f32,
    },
}
