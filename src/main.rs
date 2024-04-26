use anyhow::{Context, Result};
use clap::Parser;
use log::debug;

fn main() -> Result<()> {
    env_logger::init();
    debug!("Start parsing cli args");
    let args = minigrep::Cli::parse();
    debug!("{:?}", args.path);

    for p in &args.path.unwrap() {
        let content =
            std::fs::read_to_string(p).with_context(|| format!("Error reading file `{:?}`", p))?;

        minigrep::find_matches(&content, &args.pattern, &mut std::io::stdout())
            .with_context(|| format!("Error querying the file `{:?}`", p))?;
    }
    Ok(())
}
