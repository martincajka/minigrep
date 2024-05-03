use anyhow::Result;
use clap::Parser;
use log::debug;

fn main() -> Result<()> {
    env_logger::init();
    let args = minigrep::cli::Cli::parse();
    debug!("{:?}", args);
    let readers = minigrep::input_reader::create_readers(&args)?;
    for reader in readers {
        match args.count {
            Some(true) => {
                minigrep::find_matches_counter(reader, &args.pattern, std::io::stdout())?;
            }
            Some(false) => {
                minigrep::find_matches(reader, &args.pattern, std::io::stdout())?;
            }
            _ => (),
        }
    }

    Ok(())
}
