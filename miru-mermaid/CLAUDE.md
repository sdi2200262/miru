# miru-mermaid (library crate)

Mermaid diagram → ASCII/Unicode text renderer. Standalone library, no terminal or TUI dependencies.

## Public API

```rust
pub fn render(input: &str, opts: &RenderOptions) -> Result<String, RenderError>;

pub struct RenderOptions { pub max_width: usize, pub ascii_only: bool }

pub enum RenderError { ParseError(String), UnsupportedDiagram(String), LayoutError(String) }
```

## Pipeline

```
input string → parser::parse() → Diagram enum
  → Flowchart branch: layout::flowchart::layout() → render::flowchart::render() → Canvas
  → Sequence branch: layout::sequence::layout() → render::sequence::render() → Canvas
Canvas::to_string() → output
```

## Parser Modules

| File | Status | Handles |
|------|--------|---------|
| `parser/mod.rs` | Working | Diagram dispatch: `graph`/`flowchart` → Flowchart, `sequenceDiagram` → Sequence |
| `parser/common.rs` | Working | Shared types: `Direction`, `NodeShape` (7 variants), `EdgeStyle` (3 variants) |
| `parser/flowchart.rs` | Partial | Direction, node definitions (rectangle/diamond/rounded), edges (solid/dotted/thick), edge labels (`\|label\|`), chain syntax (`A-->B-->C`). Missing: stadium, subroutine, circle, asymmetric shapes; subgraphs; `classDef`/`style` |
| `parser/sequence.rs` | Partial | Participant declarations (`participant X as Y`), messages (solid/dashed/cross arrows), auto-registration. Missing: `activate`/`deactivate`, `Note`, `alt`/`opt`/`loop` blocks, `autonumber` |

## Layout Modules

| File | Status | Handles |
|------|--------|---------|
| `layout/mod.rs` | Working | `Position` struct (x, y, width, height) |
| `layout/flowchart.rs` | Partial | Direction-aware linear layout: TD/BT vertical stacking (centered), LR/RL horizontal stacking. No Sugiyama, no petgraph integration. |
| `layout/sequence.rs` | Partial | Message-aware column spacing, `message_ys` for row placement. `SequenceLayout` includes participant positions, message y-coords, total dimensions. |

## Render Modules

| File | Status | Handles |
|------|--------|---------|
| `render/mod.rs` | Working | Module exports |
| `render/canvas.rs` | Working | 2D char grid: `set`, `get`, `write_str`, `hline`, `vline`, `draw_box` (ASCII + Unicode), `Display` impl with trailing-space trimming |
| `render/flowchart.rs` | Partial | Node boxes with centered labels, straight-line edge routing (vertical for TD/BT, horizontal for LR/RL), arrowheads, edge labels. No L-shaped routing for non-aligned nodes. |
| `render/sequence.rs` | Partial | Participant boxes, vertical lifelines, message arrows (solid/dashed/cross) with labels. No self-message loops. |
| `render/style.rs` | Working | `BoxChars` struct with `unicode()` and `ascii()` constructors |

## Priority Work

1. **Flowchart layout** — Sugiyama implementation using petgraph (layer assignment, crossing minimization, coordinate assignment)
2. **Flowchart edge routing** — L-shaped/orthogonal paths for non-aligned nodes
3. **Sequence self-messages** — loopback arrows for same-participant messages
4. **Remaining node shapes** — stadium, subroutine, circle, asymmetric rendering in canvas

## Error Handling

- `RenderError` typed enum. Every error is classifiable — no catch-all variant.
- Error messages: lowercase, no trailing period, include the problematic value.
- No `.unwrap()` / `.expect()` outside `#[cfg(test)]`.
- `ParseError(String)` will gain structured fields (line, column) in a future iteration.
