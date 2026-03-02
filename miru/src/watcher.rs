use std::path::Path;

/// Watch a file for changes and call the callback on each change.
#[allow(dead_code)]
pub fn watch(_path: &Path, _on_change: impl Fn() + Send + 'static) -> anyhow::Result<()> {
    // TODO(tui): notify file watcher with debounce
    Ok(())
}
