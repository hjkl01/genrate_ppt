use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShapeType {
    Rectangle,
    RoundedRectangle,
    Circle,
    Line,
    Arrow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shape {
    pub shape_type: ShapeType,
}
