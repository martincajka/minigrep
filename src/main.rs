use anyhow::Result;
use clap::Parser;
use log::debug;

fn main() -> Result<()> {
    env_logger::init();
    let args = minigrep::cli::Cli::parse();
    debug!("{:?}", args);
    minigrep::process_query(&args)?;
    Ok(())
}
