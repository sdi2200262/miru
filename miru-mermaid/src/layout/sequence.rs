use std::collections::HashMap;

use crate::RenderOptions;
use crate::parser::sequence::SequenceDiagram;

use super::Position;

/// Positions for sequence diagram elements.
#[derive(Debug)]
pub struct SequenceLayout {
    /// Column positions for each participant (by ID).
    pub participants: HashMap<String, Position>,
    /// Total width and height of the diagram.
    pub width: usize,
    pub height: usize,
}

/// Lay out a sequence diagram.
///
/// Participants are arranged in columns. Messages flow top to bottom.
pub fn layout(diagram: &SequenceDiagram, _opts: &RenderOptions) -> SequenceLayout {
    let mut participant_positions = HashMap::new();
    let col_spacing = 20;
    let participant_height = 3;
    let message_row_height = 2;

    for (i, p) in diagram.participants.iter().enumerate() {
        let label_width = p.label.len() + 4;
        participant_positions.insert(
            p.id.clone(),
            Position {
                x: i * col_spacing,
                y: 0,
                width: label_width,
                height: participant_height,
            },
        );
    }

    let total_height = participant_height + (diagram.messages.len() * message_row_height) + 2;
    let total_width = diagram.participants.len() * col_spacing;

    SequenceLayout {
        participants: participant_positions,
        width: total_width,
        height: total_height,
    }
}
