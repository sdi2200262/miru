# miru

Render markdown in the terminal with mermaid diagrams as inline ASCII art.

## Status

Early development. The core pipeline works — markdown goes in, styled text with ASCII diagrams comes out (hopefully) — but most features are partial or stubbed. See the [project status table](CLAUDE.md#project-status) for details.

**What works today (hopefully):**
- Markdown rendering to stdout (`--print` mode): headings, paragraphs, lists, blockquotes, code blocks, inline formatting
- Mermaid flowchart parsing: nodes, edges, labels, chain syntax
- Mermaid sequence diagram parsing: participants, messages
- ASCII and Unicode box drawing for diagram nodes

**What's next:**
- Sugiyama graph layout for flowcharts (edge routing, layer assignment)
- Sequence diagram lifelines and message arrows
- TUI mode with scrollable viewport
- Syntax highlighting for code blocks

## Install

```
cargo install miru-cli
```

This installs the `miru` binary.

## Usage

```
miru document.md --print          # render to stdout
miru document.md --print --ascii  # ASCII-only (no Unicode box drawing)
```

TUI mode (default, without `--print`) is not yet implemented.

## Crates

| Crate | Description |
|-------|-------------|
| [`miru-cli`](https://crates.io/crates/miru-cli) | The terminal application |
| [`miru-mermaid`](https://crates.io/crates/miru-mermaid) | Standalone mermaid-to-ASCII library |

## Co-development

This project is co-developed by ([sdi2200262](https://github.com/sdi2200262)) and [Claude](https://claude.ai). All AI-assisted commits include `Co-Authored-By` attribution.

## License

MIT
