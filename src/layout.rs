use serde::{Deserialize, Serialize};

use crate::dsl::{Component, Slide, SlideType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Canvas {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionedComponent {
    pub component: Component,
    pub rect: Rect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionedSlide {
    pub slide_type: SlideType,
    pub title: String,
    pub components: Vec<PositionedComponent>,
}

pub struct LayoutEngine {
    canvas: Canvas,
}

impl Default for LayoutEngine {
    fn default() -> Self {
        Self { canvas: Canvas { width: 1280.0, height: 720.0 } }
    }
}

impl LayoutEngine {
    pub fn layout(&self, slide: &Slide) -> PositionedSlide {
        let components = match slide.slide_type {
            SlideType::Comparison => self.two_column(&slide.components),
            SlideType::Architecture => self.architecture(&slide.components),
            _ => self.default_layout(&slide.components),
        };

        PositionedSlide {
            slide_type: slide.slide_type.clone(),
            title: slide.title.clone(),
            components,
        }
    }

    fn default_layout(&self, components: &[Component]) -> Vec<PositionedComponent> {
        components.iter().enumerate().map(|(i, component)| PositionedComponent {
            component: component.clone(),
            rect: Rect { x: 80.0, y: 140.0 + i as f32 * 70.0, width: self.canvas.width - 160.0, height: 50.0 },
        }).collect()
    }

    fn two_column(&self, components: &[Component]) -> Vec<PositionedComponent> {
        components.iter().enumerate().map(|(i, component)| PositionedComponent {
            component: component.clone(),
            rect: Rect { x: if i % 2 == 0 { 80.0 } else { 660.0 }, y: 160.0 + (i / 2) as f32 * 100.0, width: 500.0, height: 80.0 },
        }).collect()
    }

    fn architecture(&self, components: &[Component]) -> Vec<PositionedComponent> {
        components.iter().enumerate().map(|(i, component)| PositionedComponent {
            component: component.clone(),
            rect: Rect { x: 150.0 + (i % 3) as f32 * 330.0, y: 220.0 + (i / 3) as f32 * 150.0, width: 220.0, height: 90.0 },
        }).collect()
    }
}
