use anyhow::{Context, Result};
use clap::{command, Parser};
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

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub count: Option<bool>,
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub line: Option<bool>,
    #[arg(short='H', long, action = clap::ArgAction::SetTrue)]
    pub heading: Option<bool>,
    #[arg(short = 'C', long, default_value_t = 2)]
    pub context: usize,
    pub pattern: String,
    pub path: Option<Vec<std::path::PathBuf>>,
}

impl Cli {
    fn read_file(&self, path: &std::path::PathBuf) -> Result<String, anyhow::Error> {
        let content: String = std::fs::read_to_string(path)
            .with_context(|| format!("Error reading file `{:?}`", path))?;

        Ok(content)
    }

    fn read_standard_input(&self) -> Result<String, anyhow::Error> {
        let mut content = String::new();
        std::io::stdin()
            .read_to_string(&mut content)
            .with_context(|| format!("Error reading standard input"))?;
        Ok(content)
    }

    pub fn process_query(&self) -> Result<(), anyhow::Error> {
        match &self.path {
            Some(paths) => {
                for p in paths {
                    let content: String = self.read_file(p)?;
                    find_matches(&content, &self.pattern, &mut std::io::stdout())?;
                }
            }
            None => {
                let content: String = self.read_standard_input()?;
                find_matches(&content, &self.pattern, &mut std::io::stdout())?;
            }
        }
        Ok(())
    }
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
