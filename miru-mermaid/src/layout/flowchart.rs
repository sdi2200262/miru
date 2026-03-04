use std::collections::HashMap;

use crate::RenderOptions;
use crate::parser::common::Direction;
use crate::parser::flowchart::Flowchart;

use super::Position;

/// Result of laying out a flowchart — positions for each node by ID.
pub type FlowchartLayout = HashMap<String, Position>;

/// Lay out a flowchart based on its direction.
///
/// TD/BT: nodes stacked vertically, centered horizontally.
/// LR/RL: nodes stacked horizontally, aligned vertically.
///
/// This is a simple linear layout. Sugiyama layering with crossing
/// minimization is not yet implemented.
pub fn layout(flowchart: &Flowchart, _opts: &RenderOptions) -> FlowchartLayout {
    // TODO(layout): implement Sugiyama layout (layer assignment, crossing
    //               minimization, coordinate assignment via petgraph)
    let mut positions = HashMap::new();
    let node_height = 3;
    let min_node_width = 14;
    let margin = 2;

    let widths: Vec<usize> = flowchart
        .nodes
        .iter()
        .map(|n| (n.label.len() + 4).max(min_node_width))
        .collect();

    let is_horizontal = matches!(
        flowchart.direction,
        Direction::LeftRight | Direction::RightLeft
    );

    if is_horizontal {
        let gap = 8; // horizontal gap between nodes for edge arrows
        let mut current_x = margin;
        for (i, node) in flowchart.nodes.iter().enumerate() {
            let width = widths[i];
            positions.insert(
                node.id.clone(),
                Position {
                    x: current_x,
                    y: margin,
                    width,
                    height: node_height,
                },
            );
            current_x += width + gap;
        }
    } else {
        let gap = 3; // vertical gap between nodes for edge arrows
        let stride = node_height + gap;
        let max_width = widths.iter().copied().max().unwrap_or(min_node_width);
        for (i, node) in flowchart.nodes.iter().enumerate() {
            let width = widths[i];
            let x = margin + (max_width - width) / 2;
            positions.insert(
                node.id.clone(),
                Position {
                    x,
                    y: i * stride,
                    width,
                    height: node_height,
                },
            );
        }
    }

    positions
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::common::NodeShape;
    use crate::parser::flowchart::Node;

    #[test]
    fn layout_horizontal() {
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
            edges: vec![],
        };
        let opts = RenderOptions::default();
        let positions = layout(&flowchart, &opts);

        let a_pos = &positions["A"];
        let b_pos = &positions["B"];
        assert!(b_pos.x > a_pos.x + a_pos.width, "B right of A");
        assert_eq!(a_pos.y, b_pos.y, "same y in LR layout");
    }

    #[test]
    fn layout_vertical_centered() {
        let flowchart = Flowchart {
            direction: Direction::TopDown,
            nodes: vec![
                Node {
                    id: "A".into(),
                    label: "Short".into(),
                    shape: NodeShape::Rectangle,
                },
                Node {
                    id: "B".into(),
                    label: "A Very Long Label".into(),
                    shape: NodeShape::Rectangle,
                },
            ],
            edges: vec![],
        };
        let opts = RenderOptions::default();
        let positions = layout(&flowchart, &opts);

        let a_pos = &positions["A"];
        let b_pos = &positions["B"];
        assert!(b_pos.y > a_pos.y + a_pos.height, "B below A");
        let a_center = a_pos.x + a_pos.width / 2;
        let b_center = b_pos.x + b_pos.width / 2;
        assert_eq!(a_center, b_center, "centers aligned vertically");
    }
}
