pub mod pptx;
pub mod shape;
pub mod text;
pub mod image;
pub mod connector;

use crate::layout::PositionedSlide;

pub trait PresentationRenderer {
    type Output;

    fn render(&self, slides: &[PositionedSlide]) -> anyhow::Result<Self::Output>;
}
