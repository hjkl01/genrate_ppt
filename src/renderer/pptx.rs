use anyhow::Result;

use crate::layout::PositionedSlide;
use super::PresentationRenderer;

/// Placeholder for the native PPTX backend.
///
/// The renderer consumes positioned semantic components from the layout engine.
/// This keeps the LLM and layout layer independent from the PPTX implementation.
#[derive(Default)]
pub struct PptxRenderer {
    pub theme: String,
}

impl PresentationRenderer for PptxRenderer {
    type Output = Vec<u8>;

    fn render(&self, slides: &[PositionedSlide]) -> Result<Self::Output> {
        // TODO:
        // 1. create pptx package
        // 2. write slide masters/theme
        // 3. convert Rect + Component into text/image/shape objects
        // 4. export pptx bytes
        //
        // Keeping this interface stable allows later replacing the backend with
        // ppt-rs or a custom OOXML writer without touching agents.
        let description = format!("slides={},theme={}", slides.len(), self.theme);
        Ok(description.into_bytes())
    }
}
