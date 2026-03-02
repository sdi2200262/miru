pub mod layout;
pub mod parser;
pub mod render;

/// Rendering options for mermaid diagrams.
#[derive(Debug, Clone)]
pub struct RenderOptions {
    /// Maximum width in characters. Diagrams will be constrained to fit.
    pub max_width: usize,
    /// Use pure ASCII instead of Unicode box-drawing characters.
    pub ascii_only: bool,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            max_width: 80,
            ascii_only: false,
        }
    }
}

/// Parse and render a mermaid diagram to ASCII/Unicode text.
///
/// Returns the rendered diagram as a multi-line string, or an error
/// if the input cannot be parsed.
pub fn render(input: &str, opts: &RenderOptions) -> Result<String, RenderError> {
    let diagram = parser::parse(input)?;

    match diagram {
        parser::Diagram::Flowchart(flowchart) => {
            let positions = layout::flowchart::layout(&flowchart, opts);
            let canvas = render::flowchart::render(&flowchart, &positions, opts);
            Ok(canvas.to_string())
        }
        parser::Diagram::Sequence(sequence) => {
            let positions = layout::sequence::layout(&sequence, opts);
            let canvas = render::sequence::render(&sequence, &positions, opts);
            Ok(canvas.to_string())
        }
    }
}

/// Errors that can occur during mermaid parsing or rendering.
#[derive(Debug)]
pub enum RenderError {
    /// The input could not be parsed as a valid mermaid diagram.
    ParseError(String),
    /// The diagram type is not yet supported.
    UnsupportedDiagram(String),
    /// The diagram could not be laid out within the given constraints.
    LayoutError(String),
}

impl std::fmt::Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseError(msg) => write!(f, "parse error: {msg}"),
            Self::UnsupportedDiagram(kind) => write!(f, "unsupported diagram type: {kind}"),
            Self::LayoutError(msg) => write!(f, "layout error: {msg}"),
        }
    }
}

impl std::error::Error for RenderError {}
