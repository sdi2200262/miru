use std::collections::HashMap;

use crate::RenderOptions;
use crate::parser::flowchart::Flowchart;

use super::Position;

/// Result of laying out a flowchart — positions for each node by ID.
pub type FlowchartLayout = HashMap<String, Position>;

/// Lay out a flowchart using a simplified Sugiyama algorithm.
///
/// Steps:
/// 1. Build a petgraph from the parsed flowchart
/// 2. Assign layers (longest-path layering)
/// 3. Order nodes within layers (crossing minimization)
/// 4. Assign coordinates (node sizing + spacing)
pub fn layout(flowchart: &Flowchart, _opts: &RenderOptions) -> FlowchartLayout {
    // TODO(layout): implement Sugiyama layout
    // For now, simple vertical stacking as a placeholder
    let mut positions = HashMap::new();
    let node_spacing_y = 4;
    let node_width = 14;
    let node_height = 3;

    for (i, node) in flowchart.nodes.iter().enumerate() {
        let label_width = node.label.len() + 4; // padding
        let width = label_width.max(node_width);
        positions.insert(
            node.id.clone(),
            Position {
                x: 2,
                y: i * node_spacing_y,
                width,
                height: node_height,
            },
        );
    }

    positions
}
