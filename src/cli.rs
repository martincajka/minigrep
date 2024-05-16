use clap::{command, Parser};

#[derive(Parser, Debug, Default)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub count: Option<bool>,
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub line: Option<bool>,
    #[arg(short='H', long, action = clap::ArgAction::SetTrue)]
    pub heading: Option<bool>,
    #[arg(short = 'C', long, default_value_t = 0)]
    pub context: usize,
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub color: Option<bool>,
    pub pattern: String,
    pub path: Option<Vec<std::path::PathBuf>>,
}
