use crate::Cli;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub struct InputReader {
    reader: Box<dyn BufRead>,
}

impl InputReader {
    pub fn from_file(path: &Path) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(Self {
            reader: Box::new(reader),
        })
    }

    pub fn from_stdin() -> Self {
        let stdin = io::stdin();
        let reader = stdin.lock();
        Self {
            reader: Box::new(reader),
        }
    }

    pub fn get_lines(self) -> io::Result<impl Iterator<Item = io::Result<String>>> {
        Ok(self.reader.lines())
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
