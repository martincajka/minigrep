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
