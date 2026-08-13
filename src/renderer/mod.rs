pub mod pptx;

use crate::layout::PositionedSlide;

pub trait PresentationRenderer {
    type Output;

    fn render(&self, slides: &[PositionedSlide]) -> anyhow::Result<Self::Output>;
}
