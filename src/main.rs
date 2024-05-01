use anyhow::Result;
use clap::Parser;
use log::debug;

fn main() -> Result<()> {
    env_logger::init();
    let args = minigrep::cli::Cli::parse();
    debug!("{:?}", args);
    let readers = minigrep::input_reader::create_readers(&args)?;
    minigrep::find_matches(readers, &args.pattern, std::io::stdout())?;
    Ok(())
}
