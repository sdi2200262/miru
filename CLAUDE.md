# miru

Terminal markdown renderer with inline mermaid ASCII diagrams. Rust workspace: `miru-cli` (binary, installs as `miru`) + `miru-mermaid` (library).

## Project Status

| Component | Crate | Status | Notes |
|-----------|-------|--------|-------|
| CLI arg parsing | miru-cli | Working | clap derive, `--print`/`--follow`/`--ascii` |
| Markdown → string | miru-cli | Partial | Headings, paragraphs, code blocks, lists, blockquotes, thematic breaks. No syntax highlighting yet. |
| TUI viewport | miru-cli | Stub | Prints message to stderr, exits |
| File watcher | miru-cli | Stub | Signature only |
| Flowchart parser | miru-mermaid | Partial | `graph`/`flowchart` + direction, nodes (rectangle/diamond/rounded), edges (solid/dotted/thick), edge labels, chain syntax |
| Sequence parser | miru-mermaid | Partial | `sequenceDiagram`, participants, messages (solid/dashed/cross) |
| Flowchart layout | miru-mermaid | Stub | Vertical stacking placeholder, no Sugiyama |
| Sequence layout | miru-mermaid | Stub | Column spacing placeholder |
| Flowchart render | miru-mermaid | Partial | Node boxes drawn, no edge routing |
| Sequence render | miru-mermaid | Partial | Participant boxes drawn, no lifelines or arrows |
| Canvas | miru-mermaid | Working | 2D char grid, box drawing (ASCII + Unicode), string writing, h/v lines |
| Style/box chars | miru-mermaid | Working | Unicode and ASCII character sets |

## Architecture

```
miru/                          # workspace root
├── Cargo.toml                 # workspace manifest, shared deps, lint config
├── rustfmt.toml               # edition = "2024"
├── LICENSE                    # MIT
├── CLAUDE.md                  # this file
│
├── miru-cli/                  # binary crate (installs as `miru`)
│   ├── Cargo.toml
│   ├── CLAUDE.md
│   └── src/
│       ├── main.rs            # CLI entry point (clap)
│       ├── app.rs             # TUI application (stub)
│       ├── watcher.rs         # file watcher (stub)
│       └── markdown/
│           ├── mod.rs          # public render() entry point
│           └── renderer.rs     # comrak AST walk → string output
│
└── miru-mermaid/              # library crate
    ├── Cargo.toml
    ├── CLAUDE.md
    └── src/
        ├── lib.rs             # public API: render(), RenderOptions, RenderError
        ├── parser/
        │   ├── mod.rs          # Diagram enum, parse() dispatcher
        │   ├── common.rs       # shared types: Direction, NodeShape, EdgeStyle
        │   ├── flowchart.rs    # flowchart parser
        │   └── sequence.rs     # sequence diagram parser
        ├── layout/
        │   ├── mod.rs          # Position type
        │   ├── flowchart.rs    # flowchart layout (stub)
        │   └── sequence.rs     # sequence layout (stub)
        └── render/
            ├── mod.rs          # module exports
            ├── canvas.rs       # 2D character grid
            ├── flowchart.rs    # flowchart → canvas
            ├── sequence.rs     # sequence → canvas
            └── style.rs        # BoxChars (Unicode/ASCII sets)
```

## Data Flow

```
1. file path         → std::fs::read_to_string
2. markdown string   → comrak::parse_document → AST
3. AST walk          → for each node:
   3a. mermaid block → miru_mermaid::render()
       3a-i.         → parser::parse() → Diagram IR
       3a-ii.        → layout::*::layout() → positions
       3a-iii.       → render::*::render() → Canvas → String
   3b. other blocks  → inline text formatting
4. assembled string  → stdout (--print) or TUI viewport
```

## Development Conventions

### Commits

Conventional Commits format:

```
type(scope): imperative description

crate/src/dir:
- file.rs: what changed

Co-Authored-By: Claude <noreply@anthropic.com>
```

Types: `feat` / `fix` / `refactor` / `test` / `docs` / `chore` / `perf`
Scopes: `miru` / `mermaid` / `parser` / `layout` / `render` / `canvas` / `tui` / `markdown` / `cli` / `ci` / `deps`

Body groups changes by directory with per-file descriptions. Single-file changes can use a flat description.

### Code Style

- `cargo fmt` (edition 2024, default settings)
- `cargo clippy --workspace` must produce zero warnings
- Workspace lints configured in root `Cargo.toml` — `unsafe_code` forbidden, pedantic clippy subset enabled

### Error Handling

**Library (`miru-mermaid`):**
- `RenderError` typed enum. No `anyhow`. No `.unwrap()` / `.expect()` outside `#[cfg(test)]`.
- Every error variant is classifiable — no catch-all `Other(String)`.
- Messages: lowercase, no trailing period, include the problematic value and location when available.

**Binary (`miru-cli`):**
- `anyhow::Result` everywhere. Convert library errors at call sites.
- A bad mermaid block displays `[mermaid error: ...]` inline with raw source as fallback — never crashes the markdown render.
- IO/encoding errors propagate to the user via anyhow.

### TODO Format

`// TODO(category): description`

Categories: `layout` / `render` / `parser` / `tui` / `perf` / `test` / `cleanup` / `ux`

### Testing

- Unit tests: `#[cfg(test)] mod tests` in the same file
- Snapshot tests: string comparison, no extra deps (adopt `insta` only if/when needed)
- `cargo test --workspace` must pass before every commit

## Communication and Writing Rules

**Chat output** (conversation with user):
- Direct, conversational prose. Markdown formatting for structure. No filler.

**File output** (CLAUDE.md, code comments, commits):
- Formal, concise, structured. Tables over prose. Lists over paragraphs.
- Every word carries information.
- Code comments explain *why*. Doc comments on public API explain *what*.

## CLAUDE.md Authoring Rules

1. State facts, not intentions. Describe what the code does, not what it is designed to do.
2. Every sentence verifiable by reading code.
3. No filler words.
4. Tables over prose. Lists over paragraphs.
5. Findable in 3 seconds — restructure if not.
6. Update in the same commit that changes module status.
7. Status vocabulary: **Working** / **Partial** / **Stub** / **Not started**. Nothing else.
8. Positive rules only. Describe the correct behavior generally; never include examples of incorrect behavior.

### Update Triggers

**Must update** (same commit):
- Module status change
- New file or module added
- Dependency added or major version changed
- Public API changed (miru-mermaid)
- New convention established

**Skip updates for:**
- Internal refactors within a module
- Bug fixes within a working module
- Patch dependency bumps
- Test additions

## Build and Run

```bash
cargo build --workspace        # compile
cargo test --workspace         # run all tests
cargo fmt --check              # formatting check
cargo clippy --workspace       # lint check
cargo run -- FILE.md --print   # render markdown to stdout
cargo run -- FILE.md --ascii   # ASCII-only mode
```

## Key Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| comrak | 0.40 | GFM markdown parser (AST) |
| ratatui | 0.30 | TUI framework |
| crossterm | 0.29 | Terminal backend for ratatui |
| petgraph | 0.8 | Graph data structure for layout |
| syntect | 5 | Syntax highlighting (not yet wired) |
| notify | 8 | File watching (not yet wired) |
| clap | 4 | CLI argument parsing |
| anyhow | 1 | Error handling in binary |
| unicode-width | 0.2 | Character width calculation |
| dirs | 6 | Platform directories |
| toml | 0.8 | Config file parsing |

## Co-Development

This project is co-developed by cobuter-man and Claude. Commits include `Co-Authored-By` attribution. CLAUDE.md files are excluded from crate publishing via `exclude` in Cargo.toml.
