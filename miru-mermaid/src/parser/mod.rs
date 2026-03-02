pub mod common;
pub mod flowchart;
pub mod sequence;

use crate::RenderError;

/// A parsed mermaid diagram.
#[derive(Debug)]
pub enum Diagram {
    Flowchart(flowchart::Flowchart),
    Sequence(sequence::SequenceDiagram),
}

/// Parse mermaid source text into a diagram IR.
pub fn parse(input: &str) -> Result<Diagram, RenderError> {
    let trimmed = input.trim();

    if trimmed.starts_with("graph ") || trimmed.starts_with("flowchart ") {
        let fc = flowchart::parse(trimmed)?;
        Ok(Diagram::Flowchart(fc))
    } else if trimmed.starts_with("sequenceDiagram") {
        let seq = sequence::parse(trimmed)?;
        Ok(Diagram::Sequence(seq))
    } else {
        let kind = trimmed.split_whitespace().next().unwrap_or("unknown");
        Err(RenderError::UnsupportedDiagram(kind.to_string()))
    }
}
