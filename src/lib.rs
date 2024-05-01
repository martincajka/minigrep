pub mod cli;

use anyhow::{Context, Result};
use cli::Cli;
use std::io::Read;

pub fn find_matches(
    content: &str,
    pattern: &str,
    mut writer: impl std::io::Write,
) -> Result<(), anyhow::Error> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
        }
    }
    Ok(())
}

fn read_file(path: &std::path::PathBuf) -> Result<String, anyhow::Error> {
    let content: String = std::fs::read_to_string(path)
        .with_context(|| format!("Error reading file `{:?}`", path))?;

    Ok(content)
}

fn read_standard_input() -> Result<String, anyhow::Error> {
    let mut content = String::new();
    std::io::stdin()
        .read_to_string(&mut content)
        .with_context(|| format!("Error reading standard input"))?;
    Ok(content)
}

pub fn process_query(cli: &Cli) -> Result<(), anyhow::Error> {
    match &cli.path {
        Some(paths) => {
            for p in paths {
                let content: String = read_file(&p)?;
                find_matches(&content, &cli.pattern, &mut std::io::stdout())?;
            }
        }
        None => {
            let content: String = read_standard_input()?;
            find_matches(&content, &cli.pattern, &mut std::io::stdout())?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use crate::find_matches;

    #[test]
    fn find_a_match() -> Result<(), anyhow::Error> {
        let mut result = Vec::new();
        find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result)?;
        assert_eq!(result, b"lorem ipsum\n");
        Ok(())
    }
}
