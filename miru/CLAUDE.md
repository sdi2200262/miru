# miru (binary crate)

CLI entry point and markdown rendering pipeline. Depends on `miru-mermaid` for diagram rendering.

## Module Map

| File | Status | Purpose |
|------|--------|---------|
| `src/main.rs` | Working | CLI parsing (clap), dispatches to `--print` or TUI |
| `src/app.rs` | Stub | TUI application — prints fallback message, exits |
| `src/watcher.rs` | Stub | File watcher signature — no implementation |
| `src/markdown/mod.rs` | Working | Public `render(source, ascii_only) → String` |
| `src/markdown/renderer.rs` | Partial | Comrak AST walk. Handles: headings, paragraphs, code blocks (mermaid dispatched to miru-mermaid, others raw), lists, blockquotes, thematic breaks, inline formatting (bold, italic, code, links). No syntax highlighting. |

## Current Limitations

- TUI mode is a stub. `--print` is the only functional output path.
- File watcher not connected. `--follow` flag is accepted but has no effect.
- No syntax highlighting for non-mermaid code blocks (syntect not wired).
- No theme loading. ASCII-only toggle works; full theming not started.

## Error Handling

- `anyhow::Result` throughout. Library `RenderError` converts at call sites.
- Mermaid parse failures render as `[mermaid error: ...]` inline — markdown render continues.
- IO errors (file not found, encoding) propagate to the user.
