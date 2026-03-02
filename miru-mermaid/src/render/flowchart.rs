use crate::RenderOptions;
use crate::layout::flowchart::FlowchartLayout;
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

    // TODO(render): draw edges with proper routing

    canvas
}
