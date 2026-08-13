use serde::{Deserialize, Serialize};

use crate::layout::PositionedSlide;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderOptions {
    pub theme: String,
    pub output: String,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self { theme: "modern".into(), output: "presentation.pptx".into() }
    }
}

pub struct PptxRenderer;

impl PptxRenderer {
    pub fn render(&self, slides: &[PositionedSlide], options: &RenderOptions) -> Vec<u8> {
        // v1 keeps the renderer boundary independent from the layout engine.
        // The next implementation will map Rect + Component into native PPTX objects.
        let manifest = serde_json::json!({
            "theme": options.theme,
            "output": options.output,
            "slides": slides,
        });
        serde_json::to_vec_pretty(&manifest).unwrap_or_default()
    }
}
