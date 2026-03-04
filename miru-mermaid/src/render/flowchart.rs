use crate::RenderOptions;
use crate::layout::Position;
use crate::layout::flowchart::FlowchartLayout;
use crate::parser::common::{Direction, EdgeStyle};
use crate::parser::flowchart::Flowchart;

use super::canvas::Canvas;

/// Render a laid-out flowchart to a character canvas.
pub fn render(flowchart: &Flowchart, positions: &FlowchartLayout, opts: &RenderOptions) -> Canvas {
    // Calculate canvas dimensions
    let mut max_x = 0usize;
    let mut max_y = 0usize;
    for pos in positions.values() {
        max_x = max_x.max(pos.x + pos.width + 2);
        max_y = max_y.max(pos.y + pos.height + 2);
    }

    let width = max_x.min(opts.max_width);
    let mut canvas = Canvas::new(width, max_y);

    // Draw nodes
    for node in &flowchart.nodes {
        if let Some(pos) = positions.get(&node.id) {
            canvas.draw_box(pos.x, pos.y, pos.width, pos.height, opts.ascii_only);

            // Center the label inside the box
            let label_x = pos.x + (pos.width.saturating_sub(node.label.len())) / 2;
            let label_y = pos.y + pos.height / 2;
            canvas.write_str(label_x, label_y, &node.label);
        }
    }

    // Draw edges
    let (v_char, h_char) = if opts.ascii_only {
        ('|', '-')
    } else {
        ('\u{2502}', '\u{2500}') // │ ─
    };

    let is_horizontal = matches!(
        flowchart.direction,
        Direction::LeftRight | Direction::RightLeft
    );

    for edge in &flowchart.edges {
        let (Some(from_pos), Some(to_pos)) = (positions.get(&edge.from), positions.get(&edge.to))
        else {
            continue;
        };

        if is_horizontal {
            draw_horizontal_edge(
                &mut canvas,
                from_pos,
                to_pos,
                &edge.style,
                &edge.label,
                h_char,
            );
        } else {
            draw_vertical_edge(
                &mut canvas,
                from_pos,
                to_pos,
                &edge.style,
                &edge.label,
                v_char,
            );
        }
    }

    canvas
}

/// Draw a vertical edge between two nodes (for TD/BT layouts).
///
/// Uses the from node's center x for a straight line. Non-aligned nodes
/// will require L-shaped routing once Sugiyama layout is implemented.
fn draw_vertical_edge(
    canvas: &mut Canvas,
    from_pos: &Position,
    to_pos: &Position,
    style: &EdgeStyle,
    label: &Option<String>,
    v_char: char,
) {
    let x = from_pos.x + from_pos.width / 2;

    let (start_y, end_y, arrow_ch) = if from_pos.y + from_pos.height <= to_pos.y {
        // from is above to → arrow points down
        (from_pos.y + from_pos.height, to_pos.y, 'v')
    } else if to_pos.y + to_pos.height <= from_pos.y {
        // to is above from → arrow points up
        (to_pos.y + to_pos.height, from_pos.y, '^')
    } else {
        return; // overlapping nodes
    };

    if start_y >= end_y {
        return;
    }

    let line_ch = match style {
        EdgeStyle::Solid | EdgeStyle::Thick => v_char,
        EdgeStyle::Dotted => ':',
    };

    for y in start_y..end_y.saturating_sub(1) {
        canvas.set(x, y, line_ch);
    }

    canvas.set(x, end_y.saturating_sub(1), arrow_ch);

    // Edge label to the right of the midpoint
    if let Some(label) = label {
        let mid_y = (start_y + end_y) / 2;
        canvas.write_str(x + 2, mid_y, label);
    }
}

/// Draw a horizontal edge between two nodes (for LR/RL layouts).
///
/// Uses the from node's center y for a straight line.
fn draw_horizontal_edge(
    canvas: &mut Canvas,
    from_pos: &Position,
    to_pos: &Position,
    style: &EdgeStyle,
    label: &Option<String>,
    h_char: char,
) {
    let y = from_pos.y + from_pos.height / 2;

    let (start_x, end_x, arrow_ch) = if from_pos.x + from_pos.width <= to_pos.x {
        // from is left of to → arrow points right
        (from_pos.x + from_pos.width, to_pos.x, '>')
    } else if to_pos.x + to_pos.width <= from_pos.x {
        // to is left of from → arrow points left
        (to_pos.x + to_pos.width, from_pos.x, '<')
    } else {
        return; // overlapping nodes
    };

    if start_x >= end_x {
        return;
    }

    let line_ch = match style {
        EdgeStyle::Solid => h_char,
        EdgeStyle::Thick => '=',
        EdgeStyle::Dotted => h_char,
    };

    match style {
        EdgeStyle::Dotted => {
            for (offset, x) in (start_x..end_x.saturating_sub(1)).enumerate() {
                if offset % 2 == 0 {
                    canvas.set(x, y, line_ch);
                }
            }
        }
        EdgeStyle::Solid | EdgeStyle::Thick => {
            for x in start_x..end_x.saturating_sub(1) {
                canvas.set(x, y, line_ch);
            }
        }
    }

    canvas.set(end_x.saturating_sub(1), y, arrow_ch);

    // Edge label above the midpoint
    if let Some(label) = label {
        let mid_x = (start_x + end_x) / 2;
        let label_x = mid_x.saturating_sub(label.len() / 2);
        if y > 0 {
            canvas.write_str(label_x, y - 1, label);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::common::NodeShape;
    use crate::parser::flowchart::{Edge, Flowchart, Node};

    #[test]
    fn render_vertical_edges() {
        let flowchart = Flowchart {
            direction: Direction::TopDown,
            nodes: vec![
                Node {
                    id: "A".into(),
                    label: "Start".into(),
                    shape: NodeShape::Rectangle,
                },
                Node {
                    id: "B".into(),
                    label: "End".into(),
                    shape: NodeShape::Rectangle,
                },
            ],
            edges: vec![Edge {
                from: "A".into(),
                to: "B".into(),
                label: None,
                style: EdgeStyle::Solid,
            }],
        };
        let opts = RenderOptions::default();
        let positions = crate::layout::flowchart::layout(&flowchart, &opts);
        let canvas = render(&flowchart, &positions, &opts);
        let output = canvas.to_string();

        assert!(output.contains("Start"));
        assert!(output.contains("End"));
        assert!(output.contains('v'), "should have downward arrowhead");
    }

    #[test]
    fn render_horizontal_edges() {
        let flowchart = Flowchart {
            direction: Direction::LeftRight,
            nodes: vec![
                Node {
                    id: "A".into(),
                    label: "Start".into(),
                    shape: NodeShape::Rectangle,
                },
                Node {
                    id: "B".into(),
                    label: "End".into(),
                    shape: NodeShape::Rectangle,
                },
            ],
            edges: vec![Edge {
                from: "A".into(),
                to: "B".into(),
                label: None,
                style: EdgeStyle::Solid,
            }],
        };
        let opts = RenderOptions::default();
        let positions = crate::layout::flowchart::layout(&flowchart, &opts);
        let canvas = render(&flowchart, &positions, &opts);
        let output = canvas.to_string();

        assert!(output.contains("Start"));
        assert!(output.contains("End"));
        assert!(output.contains('>'), "should have right arrowhead");
    }

    #[test]
    fn render_edge_with_label() {
        let flowchart = Flowchart {
            direction: Direction::TopDown,
            nodes: vec![
                Node {
                    id: "A".into(),
                    label: "Start".into(),
                    shape: NodeShape::Rectangle,
                },
                Node {
                    id: "B".into(),
                    label: "End".into(),
                    shape: NodeShape::Rectangle,
                },
            ],
            edges: vec![Edge {
                from: "A".into(),
                to: "B".into(),
                label: Some("Yes".into()),
                style: EdgeStyle::Solid,
            }],
        };
        let opts = RenderOptions::default();
        let positions = crate::layout::flowchart::layout(&flowchart, &opts);
        let canvas = render(&flowchart, &positions, &opts);
        let output = canvas.to_string();

        assert!(output.contains("Yes"), "should have edge label");
    }
}
