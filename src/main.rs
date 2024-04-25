use anyhow::{Context, Result};
use clap::Parser;
use log::debug;

fn main() -> Result<()> {
    env_logger::init();
    debug!("Start parsing cli args");
    let args = minigrep::Cli::parse();
    debug!("{:?}", args);

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Error reading file `{:?}`", &args.path))?;

    minigrep::find_matches(&content, &args.pattern, &mut std::io::stdout())
        .with_context(|| format!("Error querying the file `{:?}`", &args.path))?;
    Ok(())
}
