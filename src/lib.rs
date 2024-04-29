use clap::{command, Parser};

pub fn find_matches(
    content: &str,
    pattern: &str,
    mut writer: impl std::io::Write,
) -> Result<(), std::io::Error> {
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
    pub pattern: String,
    pub path: Option<Vec<std::path::PathBuf>>,
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
