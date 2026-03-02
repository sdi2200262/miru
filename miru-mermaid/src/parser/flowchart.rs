use super::common::{Direction, EdgeStyle, NodeShape};
use crate::RenderError;

/// A parsed flowchart diagram.
#[derive(Debug)]
pub struct Flowchart {
    pub direction: Direction,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

/// A node in the flowchart.
#[derive(Debug, Clone)]
pub struct Node {
    pub id: String,
    pub label: String,
    pub shape: NodeShape,
}

/// An edge between two nodes.
#[derive(Debug, Clone)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub label: Option<String>,
    pub style: EdgeStyle,
}

/// Parse a flowchart from mermaid source.
///
/// Expects input starting with `graph` or `flowchart` followed by a direction.
pub fn parse(input: &str) -> Result<Flowchart, RenderError> {
    let mut lines = input.lines();

    // First line: "graph TD" or "flowchart LR" etc.
    let header = lines
        .next()
        .ok_or_else(|| RenderError::ParseError("empty input".into()))?
        .trim();

    let direction = parse_direction(header)?;

    let mut nodes: Vec<Node> = Vec::new();
    let mut edges: Vec<Edge> = Vec::new();

    for line in lines {
        let line = line.trim();
        if line.is_empty() || line.starts_with("%%") {
            continue;
        }

        // Try to parse as edge(s) — lines containing arrow operators
        if let Some(parsed_edges) = try_parse_edge_chain(line) {
            for (from_raw, to_raw, label, style) in parsed_edges {
                let from_node = parse_node_ref(&from_raw);
                let to_node = parse_node_ref(&to_raw);

                // Register nodes if they have definitions
                ensure_node(&mut nodes, &from_node);
                ensure_node(&mut nodes, &to_node);

                edges.push(Edge {
                    from: from_node.id.clone(),
                    to: to_node.id.clone(),
                    label,
                    style,
                });
            }
        } else {
            // Standalone node definition
            let node = parse_node_ref(line);
            ensure_node(&mut nodes, &node);
        }
    }

    Ok(Flowchart {
        direction,
        nodes,
        edges,
    })
}

fn parse_direction(header: &str) -> Result<Direction, RenderError> {
    let parts: Vec<&str> = header.split_whitespace().collect();
    if parts.len() < 2 {
        return Err(RenderError::ParseError(
            "expected direction after graph/flowchart keyword".into(),
        ));
    }

    match parts[1] {
        "TD" | "TB" => Ok(Direction::TopDown),
        "BT" => Ok(Direction::BottomUp),
        "LR" => Ok(Direction::LeftRight),
        "RL" => Ok(Direction::RightLeft),
        other => Err(RenderError::ParseError(format!(
            "unknown direction: {other}"
        ))),
    }
}

/// Parse a node reference like `A`, `A[Label]`, `A{Decision}`, `A(Rounded)`, etc.
fn parse_node_ref(s: &str) -> Node {
    let s = s.trim().trim_end_matches(';');

    // Find the first shape delimiter
    if let Some(pos) = s.find('[') {
        let id = s[..pos].trim().to_string();
        let label = extract_between(s, pos, '[', ']');
        return Node {
            id,
            label,
            shape: NodeShape::Rectangle,
        };
    }
    if let Some(pos) = s.find('{') {
        let id = s[..pos].trim().to_string();
        let label = extract_between(s, pos, '{', '}');
        return Node {
            id,
            label,
            shape: NodeShape::Diamond,
        };
    }
    if let Some(pos) = s.find('(') {
        let id = s[..pos].trim().to_string();
        let label = extract_between(s, pos, '(', ')');
        return Node {
            id,
            label,
            shape: NodeShape::RoundedRect,
        };
    }

    // Plain ID — label is the same as the ID
    let id = s.trim().to_string();
    Node {
        label: id.clone(),
        id,
        shape: NodeShape::Rectangle,
    }
}

fn extract_between(s: &str, open_pos: usize, _open: char, close: char) -> String {
    let after_open = &s[open_pos + 1..];
    if let Some(close_pos) = after_open.rfind(close) {
        after_open[..close_pos].trim().to_string()
    } else {
        after_open.trim().to_string()
    }
}

/// Parsed edge: (from, to, label, style).
type ParsedEdge = (String, String, Option<String>, EdgeStyle);

/// Try to parse a line as a chain of edges: `A --> B --> C`
fn try_parse_edge_chain(line: &str) -> Option<Vec<ParsedEdge>> {
    // First check: does this line contain any arrow at all?
    find_next_arrow(line)?;

    // Split into segments by finding arrows iteratively.
    // For `A --> B --> C` we get edges: (A, B) and (B, C).
    let mut results = Vec::new();
    let mut remaining = line;

    while let Some((arrow_pos, arrow_len, style, label)) = find_next_arrow(remaining) {
        let from_raw = remaining[..arrow_pos].trim().to_string();
        remaining = &remaining[arrow_pos + arrow_len..];

        // The "to" node is everything up to the next arrow, or the rest of the line.
        let to_raw = match find_next_arrow(remaining) {
            Some((next_pos, _, _, _)) => remaining[..next_pos].trim().to_string(),
            None => remaining.trim().trim_end_matches(';').to_string(),
        };

        if from_raw.is_empty() || to_raw.is_empty() {
            return None;
        }

        results.push((from_raw, to_raw, label, style));
    }

    if results.is_empty() {
        None
    } else {
        Some(results)
    }
}

/// Find the next arrow operator in the string.
/// Returns (position, length, style, label).
fn find_next_arrow(s: &str) -> Option<(usize, usize, EdgeStyle, Option<String>)> {
    // Check for labeled arrows: -->|label| or ==>|label| or -.->|label|
    // Check for dotted: -.->
    if let Some(pos) = s.find("-.->") {
        let label = extract_arrow_label(s, pos + 4);
        let extra = label.as_ref().map_or(0, |l| l.len() + 2); // +2 for ||
        return Some((pos, 4 + extra, EdgeStyle::Dotted, label));
    }
    // Check for thick: ==>
    if let Some(pos) = s.find("==>") {
        let label = extract_arrow_label(s, pos + 3);
        let extra = label.as_ref().map_or(0, |l| l.len() + 2);
        return Some((pos, 3 + extra, EdgeStyle::Thick, label));
    }
    // Check for solid: -->
    if let Some(pos) = s.find("-->") {
        let label = extract_arrow_label(s, pos + 3);
        let extra = label.as_ref().map_or(0, |l| l.len() + 2);
        return Some((pos, 3 + extra, EdgeStyle::Solid, label));
    }
    None
}

/// Extract `|label|` immediately after an arrow.
fn extract_arrow_label(s: &str, after_arrow: usize) -> Option<String> {
    let rest = s.get(after_arrow..)?.trim_start();
    if let Some(inner) = rest.strip_prefix('|')
        && let Some(end) = inner.find('|')
    {
        return Some(inner[..end].trim().to_string());
    }
    None
}

/// Ensure a node exists in the list. If it already exists, update its label/shape
/// if the new definition provides more info.
fn ensure_node(nodes: &mut Vec<Node>, new: &Node) {
    if let Some(existing) = nodes.iter_mut().find(|n| n.id == new.id) {
        // Update if the new node has an explicit label (not just the ID)
        if new.label != new.id {
            existing.label = new.label.clone();
            existing.shape = new.shape.clone();
        }
    } else {
        nodes.push(new.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_flowchart() {
        let input = "graph TD\n    A[Start] --> B[End]";
        let fc = parse(input).unwrap();
        assert_eq!(fc.nodes.len(), 2);
        assert_eq!(fc.edges.len(), 1);
        assert_eq!(fc.nodes[0].label, "Start");
        assert_eq!(fc.nodes[1].label, "End");
    }

    #[test]
    fn parse_direction() {
        let input = "graph LR\n    A --> B";
        let fc = parse(input).unwrap();
        assert_eq!(fc.direction, Direction::LeftRight);
    }

    #[test]
    fn parse_diamond_node() {
        let input = "graph TD\n    A{Decision}";
        let fc = parse(input).unwrap();
        assert_eq!(fc.nodes[0].shape, NodeShape::Diamond);
        assert_eq!(fc.nodes[0].label, "Decision");
    }
}
