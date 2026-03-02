use comrak::nodes::NodeValue;
use comrak::{Arena, Options, parse_document};

use miru_mermaid::RenderOptions;

/// Walk the comrak AST and produce a rendered string.
pub fn render_to_string(source: &str, ascii_only: bool) -> String {
    let arena = Arena::new();
    let options = Options::default();
    let root = parse_document(&arena, source, &options);

    let mut output = String::new();
    let mermaid_opts = RenderOptions {
        max_width: 80,
        ascii_only,
    };

    for node in root.children() {
        render_node(node, &mermaid_opts, &mut output);
    }

    output
}

fn render_node<'a>(
    node: &'a comrak::nodes::AstNode<'a>,
    mermaid_opts: &RenderOptions,
    output: &mut String,
) {
    let data = node.data.borrow();

    match &data.value {
        NodeValue::Heading(heading) => {
            let level = heading.level;
            let prefix = "#".repeat(level as usize);
            output.push_str(&format!("{prefix} "));
            // Render inline children
            for child in node.children() {
                render_inline(child, output);
            }
            output.push('\n');
            output.push('\n');
        }

        NodeValue::Paragraph => {
            for child in node.children() {
                render_inline(child, output);
            }
            output.push('\n');
            output.push('\n');
        }

        NodeValue::CodeBlock(code_block) => {
            let info = &code_block.info;
            let literal = &code_block.literal;

            if info.trim() == "mermaid" {
                match miru_mermaid::render(literal.trim(), mermaid_opts) {
                    Ok(diagram) => {
                        output.push_str(&diagram);
                        output.push('\n');
                    }
                    Err(e) => {
                        output.push_str(&format!("[mermaid error: {e}]\n"));
                        output.push_str(literal);
                    }
                }
            } else {
                // TODO(render): syntect highlighting
                output.push_str("```");
                output.push_str(info);
                output.push('\n');
                output.push_str(literal);
                output.push_str("```\n");
            }
            output.push('\n');
        }

        NodeValue::List(_) => {
            for child in node.children() {
                render_node(child, mermaid_opts, output);
            }
        }

        NodeValue::Item(_) => {
            output.push_str("  - ");
            for child in node.children() {
                render_inline_block(child, output);
            }
            output.push('\n');
        }

        NodeValue::BlockQuote => {
            output.push_str("> ");
            for child in node.children() {
                render_inline_block(child, output);
            }
            output.push('\n');
            output.push('\n');
        }

        NodeValue::ThematicBreak => {
            output.push_str("────────────────────────────────\n\n");
        }

        _ => {
            // Recurse into unknown block nodes
            for child in node.children() {
                render_node(child, mermaid_opts, output);
            }
        }
    }
}

fn render_inline<'a>(node: &'a comrak::nodes::AstNode<'a>, output: &mut String) {
    let data = node.data.borrow();

    match &data.value {
        NodeValue::Text(text) => {
            output.push_str(text);
        }
        NodeValue::SoftBreak => {
            output.push(' ');
        }
        NodeValue::LineBreak => {
            output.push('\n');
        }
        NodeValue::Code(code) => {
            output.push('`');
            output.push_str(&code.literal);
            output.push('`');
        }
        NodeValue::Strong => {
            output.push_str("**");
            for child in node.children() {
                render_inline(child, output);
            }
            output.push_str("**");
        }
        NodeValue::Emph => {
            output.push('_');
            for child in node.children() {
                render_inline(child, output);
            }
            output.push('_');
        }
        NodeValue::Link(link) => {
            output.push('[');
            for child in node.children() {
                render_inline(child, output);
            }
            output.push_str("](");
            output.push_str(&link.url);
            output.push(')');
        }
        _ => {
            for child in node.children() {
                render_inline(child, output);
            }
        }
    }
}

fn render_inline_block<'a>(node: &'a comrak::nodes::AstNode<'a>, output: &mut String) {
    let data = node.data.borrow();
    match &data.value {
        NodeValue::Paragraph => {
            for child in node.children() {
                render_inline(child, output);
            }
        }
        _ => {
            render_inline(node, output);
        }
    }
}
