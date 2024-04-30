use anyhow::Result;
use clap::Parser;
use log::debug;

fn main() -> Result<()> {
    env_logger::init();
    let args = minigrep::Cli::parse();
    debug!("{:?}", args);
    args.process_query()?;
    Ok(())
}
