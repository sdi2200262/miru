use std::collections::HashMap;

use crate::RenderOptions;
use crate::parser::sequence::SequenceDiagram;

use super::Position;

/// Positions for sequence diagram elements.
#[derive(Debug)]
pub struct SequenceLayout {
    /// Column positions for each participant (by ID).
    pub participants: HashMap<String, Position>,
    /// Y position for each message (parallel to `SequenceDiagram::messages`).
    pub message_ys: Vec<usize>,
    /// Total width and height of the diagram.
    pub width: usize,
    pub height: usize,
}

/// Lay out a sequence diagram.
///
/// Participants are spaced by the widest box and longest message label.
/// Messages flow top to bottom with uniform row height.
pub fn layout(diagram: &SequenceDiagram, _opts: &RenderOptions) -> SequenceLayout {
    let participant_height = 3;
    let message_row_height = 2;
    let box_padding = 4; // 2 chars on each side inside box
    let left_margin = 1;

    if diagram.participants.is_empty() {
        return SequenceLayout {
            participants: HashMap::new(),
            message_ys: Vec::new(),
            width: 40,
            height: participant_height + 2,
        };
    }

    // Box widths from label lengths
    let box_widths: Vec<usize> = diagram
        .participants
        .iter()
        .map(|p| (p.label.len() + box_padding).max(8))
        .collect();

    let max_box_width = box_widths.iter().copied().max().unwrap_or(8);

    // Max message label length drives minimum column spacing
    let max_msg_label = diagram
        .messages
        .iter()
        .map(|m| m.label.len())
        .max()
        .unwrap_or(0);

    // Center-to-center distance between adjacent participants
    let col_spacing = (max_box_width + 4).max(max_msg_label + 6).max(20);

    // Place participants at uniform center spacing
    let mut participant_positions = HashMap::new();
    for (i, p) in diagram.participants.iter().enumerate() {
        let center_x = left_margin + max_box_width / 2 + i * col_spacing;
        let x = center_x.saturating_sub(box_widths[i] / 2);
        participant_positions.insert(
            p.id.clone(),
            Position {
                x,
                y: 0,
                width: box_widths[i],
                height: participant_height,
            },
        );
    }

    // Message y positions
    let message_ys: Vec<usize> = (0..diagram.messages.len())
        .map(|i| participant_height + 1 + i * message_row_height)
        .collect();

    let total_height = message_ys
        .last()
        .map_or(participant_height + 2, |&last_y| last_y + 2);

    let last_center =
        left_margin + max_box_width / 2 + (diagram.participants.len() - 1) * col_spacing;
    let last_half = box_widths.last().copied().unwrap_or(8) / 2;
    let total_width = last_center + last_half + 2;

    SequenceLayout {
        participants: participant_positions,
        message_ys,
        width: total_width,
        height: total_height,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::sequence::{Message, MessageStyle, Participant};

    #[test]
    fn layout_two_participants() {
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
        let result = layout(&diagram, &opts);

        assert_eq!(result.participants.len(), 2);
        assert_eq!(result.message_ys.len(), 1);

        let a_pos = &result.participants["A"];
        let b_pos = &result.participants["B"];
        assert!(
            b_pos.x > a_pos.x + a_pos.width,
            "participants should not overlap"
        );
    }

    #[test]
    fn layout_empty_diagram() {
        let diagram = SequenceDiagram {
            participants: vec![],
            messages: vec![],
        };
        let opts = RenderOptions::default();
        let result = layout(&diagram, &opts);

        assert_eq!(result.participants.len(), 0);
        assert!(result.width > 0);
        assert!(result.height > 0);
    }

    #[test]
    fn message_ys_increase_monotonically() {
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
            messages: vec![
                Message {
                    from: "A".into(),
                    to: "B".into(),
                    label: "one".into(),
                    style: MessageStyle::Solid,
                },
                Message {
                    from: "B".into(),
                    to: "A".into(),
                    label: "two".into(),
                    style: MessageStyle::Dashed,
                },
            ],
        };
        let opts = RenderOptions::default();
        let result = layout(&diagram, &opts);

        assert_eq!(result.message_ys.len(), 2);
        assert!(result.message_ys[1] > result.message_ys[0]);
    }
}
