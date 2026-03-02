use crate::RenderError;

/// A parsed sequence diagram.
#[derive(Debug)]
pub struct SequenceDiagram {
    pub participants: Vec<Participant>,
    pub messages: Vec<Message>,
}

/// A participant (actor) in a sequence diagram.
#[derive(Debug, Clone)]
pub struct Participant {
    pub id: String,
    pub label: String,
}

/// A message between participants.
#[derive(Debug, Clone)]
pub struct Message {
    pub from: String,
    pub to: String,
    pub label: String,
    pub style: MessageStyle,
}

/// Style of a sequence diagram message arrow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageStyle {
    /// `->>` solid arrow
    Solid,
    /// `-->>` dashed arrow
    Dashed,
    /// `-x` solid cross (lost message)
    SolidCross,
    /// `--x` dashed cross
    DashedCross,
}

/// Parse a sequence diagram from mermaid source.
pub fn parse(input: &str) -> Result<SequenceDiagram, RenderError> {
    let mut participants: Vec<Participant> = Vec::new();
    let mut messages: Vec<Message> = Vec::new();

    for line in input.lines().skip(1) {
        let line = line.trim();
        if line.is_empty() || line.starts_with("%%") {
            continue;
        }

        if let Some(rest) = line.strip_prefix("participant ") {
            let (id, label) = parse_participant_decl(rest);
            if !participants.iter().any(|p| p.id == id) {
                participants.push(Participant { id, label });
            }
            continue;
        }

        if let Some(msg) = try_parse_message(line) {
            // Auto-register participants from messages
            ensure_participant(&mut participants, &msg.from);
            ensure_participant(&mut participants, &msg.to);
            messages.push(msg);
        }
    }

    Ok(SequenceDiagram {
        participants,
        messages,
    })
}

fn parse_participant_decl(s: &str) -> (String, String) {
    // "Alice as A" or just "Alice"
    if let Some(pos) = s.find(" as ") {
        let id = s[..pos].trim().to_string();
        let label = s[pos + 4..].trim().to_string();
        (id, label)
    } else {
        let id = s.trim().to_string();
        (id.clone(), id)
    }
}

fn try_parse_message(line: &str) -> Option<Message> {
    // Try each arrow type (longest first to avoid partial matches)
    let arrows = [
        ("-->>", MessageStyle::Dashed),
        ("->>", MessageStyle::Solid),
        ("--x", MessageStyle::DashedCross),
        ("-x", MessageStyle::SolidCross),
    ];

    for (arrow, style) in &arrows {
        if let Some(pos) = line.find(arrow) {
            let from = line[..pos].trim().to_string();
            let rest = &line[pos + arrow.len()..];

            // The rest is ": label" or just ":"
            let (to, label) = if let Some(colon_pos) = rest.find(':') {
                let to = rest[..colon_pos].trim().to_string();
                let label = rest[colon_pos + 1..].trim().to_string();
                (to, label)
            } else {
                (rest.trim().to_string(), String::new())
            };

            return Some(Message {
                from,
                to,
                label,
                style: style.clone(),
            });
        }
    }

    None
}

fn ensure_participant(participants: &mut Vec<Participant>, id: &str) {
    if !participants.iter().any(|p| p.id == id) {
        participants.push(Participant {
            id: id.to_string(),
            label: id.to_string(),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_sequence() {
        let input = "sequenceDiagram\n    Alice->>Bob: Hello\n    Bob-->>Alice: Hi back";
        let seq = parse(input).unwrap();
        assert_eq!(seq.participants.len(), 2);
        assert_eq!(seq.messages.len(), 2);
        assert_eq!(seq.messages[0].label, "Hello");
        assert_eq!(seq.messages[1].style, MessageStyle::Dashed);
    }

    #[test]
    fn parse_with_participant_decl() {
        let input = "sequenceDiagram\n    participant A as Alice\n    participant B as Bob\n    A->>B: Hello";
        let seq = parse(input).unwrap();
        assert_eq!(seq.participants[0].label, "Alice");
        assert_eq!(seq.participants[1].label, "Bob");
    }
}
