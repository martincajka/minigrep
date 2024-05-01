pub mod cli;
pub mod input_reader;

use anyhow::Result;
use cli::Cli;
use input_reader::InputReader;

pub fn find_matches(
    readers: Vec<InputReader>,
    pattern: &str,
    mut writer: impl std::io::Write,
) -> Result<(), anyhow::Error> {
    for reader in readers {
        let lines = reader.get_lines()?;
        for line_result in lines {
            let line = line_result?;
            if line.contains(pattern) {
                writeln!(writer, "{}", line)?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn find_a_match() -> Result<(), anyhow::Error> {
        Ok(())
    }
}
