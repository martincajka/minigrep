pub mod cli;
pub mod context_window;
pub mod input_reader;

use anyhow::Result;
use cli::Cli;
use colored::*;
use context_window::ContextWindow;
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
            let line = match args.color {
                Some(true) => line.replace(&args.pattern, &args.pattern.red().to_string()),
                _ => line,
            };
            let prefix = generate_prefix(&input_name, i, args);
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

    let mut context = ContextWindow::new(args.context, args.context);
    for (i, line_result) in lines.enumerate() {
        if context.is_ready_to_write_out() {
            context.write(&mut writer)?;
        }
        let mut line = line_result?;
        let is_match = line.contains(&args.pattern);
        let mut prefix = generate_prefix(&input_name, i, args);
        if is_match {
            line = match args.color {
                Some(true) => line.replace(&args.pattern, &args.pattern.red().to_string()),
                _ => line,
            };
        }
        prefix.push_str(&line);
        context.add_line(&prefix, is_match);
    }
    context.finalize_after_last_line(&mut writer)?;

    Ok(())
}

fn generate_prefix(input_name: &str, i: usize, args: &Cli) -> String {
    match (args.line, args.heading) {
        (Some(true), Some(true)) => format!("{}:{}:", input_name, i + 1),
        (Some(true), _) => format!("{}:", i + 1),
        (_, Some(true)) => format!("{}:", input_name),
        _ => String::new(),
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn find_a_match() -> Result<(), anyhow::Error> {
        Ok(())
    }
}
