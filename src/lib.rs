use clap::Parser;

pub fn find_matches(
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
pub struct Cli {
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
