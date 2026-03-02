mod app;
mod markdown;
mod watcher;

use std::path::PathBuf;

use clap::Parser;

/// Terminal markdown renderer with inline mermaid ASCII diagrams.
#[derive(Parser, Debug)]
#[command(name = "miru", version, about)]
struct Cli {
    /// Markdown file to render.
    file: PathBuf,

    /// Print rendered output to stdout instead of opening the TUI.
    #[arg(long)]
    print: bool,

    /// Watch ~/.cache/miru-current for file switches (multi-file mode).
    #[arg(long)]
    follow: bool,

    /// Use pure ASCII instead of Unicode box-drawing characters.
    #[arg(long)]
    ascii: bool,
}

#[allow(clippy::print_stdout)]
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let source = std::fs::read_to_string(&cli.file)?;
    let rendered = markdown::render(&source, cli.ascii);

    if cli.print {
        println!("{rendered}");
    } else {
        app::run(&cli.file, cli.follow)?;
    }

    Ok(())
}
