use std::io::Read;

use anyhow::{Context, Result};
use clap::Parser;
use log::debug;

fn main() -> Result<()> {
    env_logger::init();
    debug!("Start parsing cli args");
    let args = minigrep::Cli::parse();
    debug!("{:?}", args);

    let mut content = String::new();
    match args.path {
        Some(paths) => {
            for p in &paths {
                content = std::fs::read_to_string(p)
                    .with_context(|| format!("Error reading file `{:?}`", p))?;
                minigrep::find_matches(&content, &args.pattern, &mut std::io::stdout())
                    .with_context(|| format!("Error querying the file `{:?}`", p))?;
            }
        }
        None => {
            std::io::stdin().read_to_string(&mut content)?;
            minigrep::find_matches(&content, &args.pattern, &mut std::io::stdout())
                .with_context(|| "Error querying the standard input")?;
        }
    }

    Ok(())
}
