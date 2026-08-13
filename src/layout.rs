use crate::dsl::Slide;

#[derive(Debug)]
pub struct PositionedSlide {
    pub slide: Slide,
    pub width: f32,
    pub height: f32,
}

pub fn layout(slide: Slide) -> PositionedSlide {
    PositionedSlide {
        slide,
        width: 1280.0,
        height: 720.0,
    }
}
