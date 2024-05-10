pub mod cli;
pub mod context_window;
pub mod input_reader;

use anyhow::Result;
use cli::Cli;
use input_reader::InputReader;

pub fn find_matches(
    reader: InputReader,
    args: &Cli,
    mut writer: impl std::io::Write,
) -> Result<(), anyhow::Error> {
    let input_name = reader.get_input_source_name();
    let lines = reader.get_lines()?;
    for (i, line_result) in lines.enumerate() {
        let line = line_result?;
        if line.contains(&args.pattern) {
            let prefix = match (args.line, args.heading) {
                (Some(true), Some(true)) => format!("{}:{}:", input_name, i + 1),
                (Some(true), _) => format!("{}:", i + 1),
                (_, Some(true)) => format!("{}:", input_name),
                _ => String::new(),
            };
            writeln!(writer, "{}{}", prefix, line)?;
        }
    }

    Ok(())
}

pub fn find_matches_counter(
    reader: InputReader,
    args: &Cli,
    mut writer: impl std::io::Write,
) -> Result<(), anyhow::Error> {
    let input_name = reader.get_input_source_name();
    let lines = reader.get_lines()?;
    let mut count: usize = 0;
    for line_result in lines {
        let line = line_result?;
        if line.contains(&args.pattern) {
            count += 1;
        }
    }
    if let Some(true) = args.heading {
        writeln!(writer, "{}:{}", input_name, count)?;
    } else {
        writeln!(writer, "{}", count)?;
    }
    Ok(())
}

pub fn find_matches_context(
    reader: InputReader,
    args: &Cli,
    mut writer: impl std::io::Write,
) -> Result<(), anyhow::Error> {
    let input_name = reader.get_input_source_name();
    let lines = reader.get_lines()?;
    let mut count: usize = 0;
    for line_result in lines {
        let line = line_result?;
        if line.contains(&args.pattern) {
            count += 1;
        }
    }
    if let Some(true) = args.heading {
        writeln!(writer, "{}:{}", input_name, count)?;
    } else {
        writeln!(writer, "{}", count)?;
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
