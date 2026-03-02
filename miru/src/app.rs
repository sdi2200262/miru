use std::path::Path;

/// Run the TUI application.
#[allow(clippy::print_stderr)]
pub fn run(_file: &Path, _follow: bool) -> anyhow::Result<()> {
    // TODO(tui): ratatui TUI with scrollable viewport
    eprintln!("TUI mode not yet implemented. Use --print for now.");
    Ok(())
}
