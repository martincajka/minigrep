pub mod cli;
pub mod context_window;
pub mod input_reader;

use anyhow::Result;
use cli::Cli;
use input_reader::InputReader;

pub fn find_matches(
    reader: InputReader,
    pattern: &str,
    mut writer: impl std::io::Write,
) -> Result<(), anyhow::Error> {
    let lines = reader.get_lines()?;
    for line_result in lines {
        let line = line_result?;
        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
        }
    }

    Ok(())
}

pub fn find_matches_counter(
    reader: InputReader,
    pattern: &str,
    mut writer: impl std::io::Write,
) -> Result<(), anyhow::Error> {
    let lines = reader.get_lines()?;
    let mut count: usize = 0;
    for line_result in lines {
        let line = line_result?;
        if line.contains(pattern) {
            count += 1;
        }
    }
    writeln!(writer, "{}", count)?;
    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn find_a_match() -> Result<(), anyhow::Error> {
        Ok(())
    }
}
