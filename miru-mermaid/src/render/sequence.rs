use crate::RenderOptions;
use crate::layout::sequence::SequenceLayout;
use crate::parser::sequence::{MessageStyle, SequenceDiagram};

use super::canvas::Canvas;

/// Render a laid-out sequence diagram to a character canvas.
pub fn render(diagram: &SequenceDiagram, layout: &SequenceLayout, opts: &RenderOptions) -> Canvas {
    let width = layout.width.min(opts.max_width).max(40);
    let mut canvas = Canvas::new(width, layout.height);

    let (v_char, h_char) = if opts.ascii_only {
        ('|', '-')
    } else {
        ('\u{2502}', '\u{2500}') // │ ─
    };

    // Draw participant boxes at the top
    for participant in &diagram.participants {
        if let Some(pos) = layout.participants.get(&participant.id) {
            canvas.draw_box(pos.x, pos.y, pos.width, pos.height, opts.ascii_only);
            let label_x = pos.x + (pos.width.saturating_sub(participant.label.len())) / 2;
            let label_y = pos.y + pos.height / 2;
            canvas.write_str(label_x, label_y, &participant.label);
        }
    }

    // Draw lifelines below participant boxes
    for participant in &diagram.participants {
        if let Some(pos) = layout.participants.get(&participant.id) {
            let center_x = pos.x + pos.width / 2;
            let lifeline_start = pos.y + pos.height;
            for y in lifeline_start..layout.height {
                canvas.set(center_x, y, v_char);
            }
        }
    }

    // Draw message arrows
    for (i, msg) in diagram.messages.iter().enumerate() {
        if i >= layout.message_ys.len() {
            break;
        }
        let y = layout.message_ys[i];

        let from_pos = layout.participants.get(&msg.from);
        let to_pos = layout.participants.get(&msg.to);

        let (Some(from_pos), Some(to_pos)) = (from_pos, to_pos) else {
            continue;
        };

        let from_center = from_pos.x + from_pos.width / 2;
        let to_center = to_pos.x + to_pos.width / 2;

        // TODO(render): self-message loops
        if from_center == to_center {
            continue;
        }

        let going_right = from_center < to_center;
        let (left_x, right_x) = if going_right {
            (from_center + 1, to_center.saturating_sub(1))
        } else {
            (to_center + 1, from_center.saturating_sub(1))
        };

        if right_x <= left_x {
            continue;
        }

        // Label above the arrow
        if !msg.label.is_empty() && y > 0 {
            let span = right_x - left_x;
            let label_x = left_x + span.saturating_sub(msg.label.len()) / 2;
            canvas.write_str(label_x, y - 1, &msg.label);
        }

        // Arrow body (excluding arrowhead position)
        let (body_start, body_end, arrow_x) = if going_right {
            (left_x, right_x.saturating_sub(1), right_x)
        } else {
            (left_x + 1, right_x, left_x)
        };

        if body_end >= body_start {
            match msg.style {
                MessageStyle::Solid | MessageStyle::SolidCross => {
                    for x in body_start..=body_end {
                        canvas.set(x, y, h_char);
                    }
                }
                MessageStyle::Dashed | MessageStyle::DashedCross => {
                    for (offset, x) in (body_start..=body_end).enumerate() {
                        if offset % 2 == 0 {
                            canvas.set(x, y, h_char);
                        } else {
                            canvas.set(x, y, ' ');
                        }
                    }
                }
            }
        }

        // Arrowhead
        let arrow_char = match msg.style {
            MessageStyle::Solid | MessageStyle::Dashed => {
                if going_right {
                    '>'
                } else {
                    '<'
                }
            }
            MessageStyle::SolidCross | MessageStyle::DashedCross => 'x',
        };
        canvas.set(arrow_x, y, arrow_char);
    }

    canvas
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::sequence::{Message, Participant};

    #[test]
    fn render_draws_lifelines_and_arrows() {
        let diagram = SequenceDiagram {
            participants: vec![
                Participant {
                    id: "A".into(),
                    label: "Alice".into(),
                },
                Participant {
                    id: "B".into(),
                    label: "Bob".into(),
                },
            ],
            messages: vec![Message {
                from: "A".into(),
                to: "B".into(),
                label: "Hello".into(),
                style: MessageStyle::Solid,
            }],
        };
        let opts = RenderOptions::default();
        let seq_layout = crate::layout::sequence::layout(&diagram, &opts);
        let canvas = render(&diagram, &seq_layout, &opts);
        let output = canvas.to_string();

        assert!(output.contains("Alice"));
        assert!(output.contains("Bob"));
        assert!(output.contains('>'), "should have right-pointing arrow");
        assert!(output.contains("Hello"), "should have message label");
    }

    #[test]
    fn render_dashed_arrow_left() {
        let diagram = SequenceDiagram {
            participants: vec![
                Participant {
                    id: "A".into(),
                    label: "Alice".into(),
                },
                Participant {
                    id: "B".into(),
                    label: "Bob".into(),
                },
            ],
            messages: vec![Message {
                from: "B".into(),
                to: "A".into(),
                label: "Reply".into(),
                style: MessageStyle::Dashed,
            }],
        };
        let opts = RenderOptions::default();
        let seq_layout = crate::layout::sequence::layout(&diagram, &opts);
        let canvas = render(&diagram, &seq_layout, &opts);
        let output = canvas.to_string();

        assert!(output.contains('<'), "dashed left arrow should have '<'");
        assert!(output.contains("Reply"));
    }

    #[test]
    fn render_ascii_mode() {
        let diagram = SequenceDiagram {
            participants: vec![
                Participant {
                    id: "A".into(),
                    label: "A".into(),
                },
                Participant {
                    id: "B".into(),
                    label: "B".into(),
                },
            ],
            messages: vec![Message {
                from: "A".into(),
                to: "B".into(),
                label: "msg".into(),
                style: MessageStyle::Solid,
            }],
        };
        let opts = RenderOptions {
            max_width: 80,
            ascii_only: true,
        };
        let seq_layout = crate::layout::sequence::layout(&diagram, &opts);
        let canvas = render(&diagram, &seq_layout, &opts);
        let output = canvas.to_string();

        // ASCII mode: no Unicode box-drawing characters
        assert!(!output.contains('\u{250C}')); // no ┌
        assert!(!output.contains('\u{2500}')); // no ─
        assert!(output.contains('+'));
        assert!(output.contains('-'));
    }
}
