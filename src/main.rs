use anyhow::Result;
use clap::Parser;
use log::debug;

fn main() -> Result<()> {
    env_logger::init();
    let args = minigrep::cli::Cli::parse();
    debug!("{:?}", args);
    let readers = minigrep::input_reader::create_readers(&args)?;
    for reader in readers {
        match (args.count, args.context) {
            (Some(true), _) => {
                minigrep::find_matches_counter(reader, &args, std::io::stdout())?;
            }
            (Some(false), 0) => {
                minigrep::find_matches(reader, &args, std::io::stdout())?;
            }
            _ => {
                minigrep::find_matches_context(reader, &args, std::io::stdout())?;
            }
        }
    }

    Ok(())
}
