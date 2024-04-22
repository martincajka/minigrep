use anyhow::{Context, Result};
use clap::Parser;
use log::info;

fn main() -> Result<()> {
    env_logger::init();
    info!("Starting up");
    let args = Cli::parse();

    info!("pattern: {:?}, path: {:?}", args.pattern, args.path);

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Error reading `{:?}`", &args.path))?;

    find_matches(&content, &args.pattern, &mut std::io::stdout())
        .with_context(|| format!("Error querying the file `{:?}`", &args.path))?;
    Ok(())
}

fn find_matches(
    content: &str,
    pattern: &str,
    mut writer: impl std::io::Write,
) -> Result<(), std::io::Error> {
    for line in content.lines() {
        if line.contains(pattern) {
            return writeln!(writer, "{}", line);
        }
    }
    Ok(())
}

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

#[cfg(test)]
mod tests {
    use std::io;

    use crate::find_matches;

    #[test]
    fn find_a_match() -> io::Result<()> {
        let mut result = Vec::new();
        find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result)?;
        assert_eq!(result, b"lorem ipsum\n");
        Ok(())
    }
}
