use crate::RenderOptions;
use crate::layout::sequence::SequenceLayout;
use crate::parser::sequence::SequenceDiagram;

use super::canvas::Canvas;

/// Render a laid-out sequence diagram to a character canvas.
pub fn render(diagram: &SequenceDiagram, layout: &SequenceLayout, opts: &RenderOptions) -> Canvas {
    let width = layout.width.min(opts.max_width).max(40);
    let mut canvas = Canvas::new(width, layout.height);

    // Draw participant boxes at the top
    for participant in &diagram.participants {
        if let Some(pos) = layout.participants.get(&participant.id) {
            canvas.draw_box(pos.x, pos.y, pos.width, pos.height, opts.ascii_only);
            let label_x = pos.x + (pos.width.saturating_sub(participant.label.len())) / 2;
            let label_y = pos.y + pos.height / 2;
            canvas.write_str(label_x, label_y, &participant.label);
        }
    }

    // TODO(render): draw lifelines and message arrows

    canvas
}
