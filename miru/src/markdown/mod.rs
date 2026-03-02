mod renderer;

/// Render a markdown string to styled terminal text.
///
/// Mermaid code blocks are detected and rendered as inline ASCII diagrams
/// via the miru-mermaid engine.
pub fn render(source: &str, ascii_only: bool) -> String {
    renderer::render_to_string(source, ascii_only)
}
