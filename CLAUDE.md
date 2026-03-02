# miru

Terminal markdown renderer with inline mermaid ASCII diagrams. Rust workspace: `miru-cli` (binary, installs as `miru`) + `miru-mermaid` (library).

## Publishing

| Crate | crates.io | Version | Notes |
|-------|-----------|---------|-------|
| `miru-cli` | [crates.io/crates/miru-cli](https://crates.io/crates/miru-cli) | 0.0.1 | Name reservation. Installs as `miru` binary. |
| `miru-mermaid` | [crates.io/crates/miru-mermaid](https://crates.io/crates/miru-mermaid) | 0.0.1 | Name reservation. |

`miru` on crates.io is taken (unrelated project). The project name is still **miru** everywhere (repo, binary, docs) — only the crate package name is `miru-cli`.

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

---

# Session Memory System

This project uses an automated session memory system. Follow these instructions throughout every session.

## At Session Start

If `.claude/memory/index.md` exists, read it. It lists recent sessions and archives — use it to orient yourself without reading full logs unless needed.

## Memory Log Location and Naming

Session memory logs are stored in `.claude/memory/` with this naming convention:

```
session-YYYY-MM-DD-NNN.md
```

Where `NNN` is derived by counting existing `session-*.md` files directly in `.claude/memory/` (not in `archive/`) and adding 1, zero-padded to 3 digits. The session ID is stored in the frontmatter, not the filename.

## When to Write Memory

The cc-context-awareness system injects reminders at 50%, 65%, and 80% context usage. Each reminder includes your current session ID. When you receive one:

- **First reminder (50%)**: Check `.claude/memory/` for a file with `session_id: <your session ID>` in its frontmatter. If none exists, create a new one using the counter. Also update `index.md`.
- **Later reminders (65%, 80%)**: Find your existing log for this session and append to it.

## Creating a New Log

1. Count existing `session-*.md` files directly in `.claude/memory/` (not in `archive/`)
2. Add 1, zero-pad to 3 digits (e.g. `007`)
3. Create `.claude/memory/session-YYYY-MM-DD-NNN.md`

## Memory Log Format

```markdown
---
date: YYYY-MM-DD
session_id: <full session ID from the reminder>
context_at_log: <percentage>%
---

## Current Work
[What task or project is being worked on and its current state]

## Completed This Session
[What was accomplished — specific files, features, fixes]

## Key Decisions
[Important technical or design choices made, and why]

## Files Modified
[Key files created or changed with brief descriptions]
- `path/to/file.ts` — description of change

## In Progress
[Anything started but not finished]

## Next Steps
[Specific actions to take next session — enough detail to continue immediately]

## Notes
[User preferences, known issues, environment details, other context]
```

Subsequent updates append with a separator:

```markdown
---
*Updated at 65% context*

[Updated sections only]
```

## Session Index (`index.md`)

`.claude/memory/index.md` is a running index of all sessions. When creating a new session log (at 50%), add or update the row for this session:

```markdown
# Session Memory Index

| File | Date | Summary |
|------|------|---------|
| session-YYYY-MM-DD-NNN.md | YYYY-MM-DD | One-sentence summary of current work |

## Archives
| Archive | Covers |
|---------|--------|
| archive-YYYY-MM-DD.md | sessions NNN–NNN (date range) |
```

Keep the most recent sessions at the top of the table. The Archives section is managed by the memory-archiver agent — do not edit it manually.

## After Compaction

When resuming after compaction, a hook automatically loads the most recent session log (or archive if no individual log exists) as context. The `session_id` in that log belongs to the previous session — your current session has a different ID. Create a new log for your current session using the counter, and update `index.md`.

## Archival

When 5 session logs accumulate, a hook injects archival instructions at the start of the next post-compaction session. When you see these instructions, delegate to the `memory-archiver` agent (defined in `.claude/agents/memory-archiver.md`) to synthesize the older logs into `.claude/memory/archive/`. The newest session log is always preserved — never archived — so the previous session's context is never lost.

## Integration with Claude Code Auto-Memory

Claude Code's native auto-memory (`MEMORY.md` in `~/.claude/projects/<project>/memory/`) and this session memory system serve **different purposes and different namespaces** — they do not conflict:

| System | Location | Purpose | Loaded |
|--------|----------|---------|--------|
| Native auto-memory | `~/.claude/projects/.../memory/MEMORY.md` | Stable cross-session knowledge: preferences, conventions, architecture | Every session start (auto) |
| Session logs | `.claude/memory/session-*.md` | Per-session work history: what was done, decisions, next steps | After compaction (via hook), or on-demand |
| Session index | `.claude/memory/index.md` | Index of all sessions for quick orientation | On-demand (read at session start per instructions above) |

**Synergy:** If Claude Code auto-memory is active and you write to `MEMORY.md` this session, add a brief pointer:

```markdown
## Session Memory
Session-specific work logs at `.claude/memory/` — see `index.md` for history.
```

This ensures every fresh session (not just post-compaction) sees a reminder to check the session index. Keep it brief — `MEMORY.md` has a 200-line display limit.
