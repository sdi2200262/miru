pub mod flowchart;
pub mod sequence;

/// A positioned element on the canvas.
#[derive(Debug, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}
