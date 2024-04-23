use anyhow::{Context, Result};
use clap::Parser;
use log::info;

fn main() -> Result<()> {
    env_logger::init();
    info!("Starting up");
    let args = minigrep::Cli::parse();

    info!("pattern: {:?}, path: {:?}", args.pattern, args.path);

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Error reading `{:?}`", &args.path))?;

    minigrep::find_matches(&content, &args.pattern, &mut std::io::stdout())
        .with_context(|| format!("Error querying the file `{:?}`", &args.path))?;
    Ok(())
}
