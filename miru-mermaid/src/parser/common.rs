/// Direction of a flowchart.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    TopDown,
    BottomUp,
    LeftRight,
    RightLeft,
}

/// Shape of a flowchart node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeShape {
    /// `[text]` — rectangle
    Rectangle,
    /// `(text)` — rounded rectangle
    RoundedRect,
    /// `{text}` — diamond / decision
    Diamond,
    /// `([text])` — stadium / pill
    Stadium,
    /// `[[text]]` — subroutine
    Subroutine,
    /// `>text]` — asymmetric / flag
    Asymmetric,
    /// `((text))` — circle
    Circle,
}

/// Style of an edge arrow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EdgeStyle {
    /// `-->` solid arrow
    Solid,
    /// `-.->` dotted arrow
    Dotted,
    /// `==>` thick arrow
    Thick,
}
