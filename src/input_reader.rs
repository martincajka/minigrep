use crate::Cli;
use std::ffi::OsString;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub struct InputReader {
    reader: Box<dyn BufRead>,
    input_source_name: OsString,
}

impl InputReader {
    pub fn from_file(path: &Path) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(Self {
            reader: Box::new(reader),
            input_source_name: path.as_os_str().to_os_string(),
        })
    }

    pub fn from_stdin() -> Self {
        let stdin = io::stdin();
        let reader = stdin.lock();
        Self {
            reader: Box::new(reader),
            input_source_name: OsString::from("(standard input)"),
        }
    }

    pub fn get_lines(self) -> io::Result<impl Iterator<Item = io::Result<String>>> {
        Ok(self.reader.lines())
    }

    pub fn get_input_source_name(&self) -> String {
        self.input_source_name
            .clone()
            .to_string_lossy()
            .into_owned()
    }
}

pub fn create_readers(cli: &Cli) -> io::Result<Vec<InputReader>> {
    match &cli.path {
        Some(paths) if !paths.is_empty() => paths
            .iter()
            .map(|path| InputReader::from_file(path))
            .collect(),
        _ => Ok(vec![InputReader::from_stdin()]),
    }
}

#[cfg(test)]
mod tests {
    use assert_fs::fixture::FileWriteStr;

    use super::*;

    #[test]
    fn test_input_reader_from_file() -> Result<(), Box<dyn std::error::Error>> {
        let temp_file = assert_fs::NamedTempFile::new("test.txt")?;
        temp_file.write_str("Hello, world")?;

        let input_reader = InputReader::from_file(temp_file.path())?;
        let lines: Vec<_> = input_reader.get_lines()?.collect::<Result<_, _>>()?;
        assert_eq!(lines, vec!["Hello, world"]);

        Ok(())
    }

    #[test]
    fn test_input_reader_from_stdin() {
        let input_reader = InputReader::from_stdin();
        assert_eq!(input_reader.get_input_source_name(), "(standard input)");
    }

    #[test]
    fn test_create_readers_from_file() -> Result<(), Box<dyn std::error::Error>> {
        let temp_file = assert_fs::NamedTempFile::new("test.txt")?;
        temp_file.write_str("Hello, world")?;

        let cli = Cli {
            path: Some(vec![temp_file.path().to_path_buf()]),
            ..Default::default()
        };

        let readers = create_readers(&cli)?;
        assert_eq!(readers.len(), 1);
        assert_eq!(
            readers[0].get_input_source_name(),
            temp_file.path().to_string_lossy()
        );

        Ok(())
    }

    #[test]
    fn test_create_readers_from_stdin() -> io::Result<()> {
        let cli = Cli {
            path: None,
            ..Default::default()
        };

        let readers = create_readers(&cli)?;
        assert_eq!(readers.len(), 1);
        assert_eq!(readers[0].get_input_source_name(), "(standard input)");

        Ok(())
    }
}
