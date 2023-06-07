use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long)]
    pub path: PathBuf,

    #[clap(short = 't', long,  value_enum, default_value_t=NewLine::LF)]
    pub newline: NewLine,

    #[clap(short, long, default_value_t = false)]
    pub write: bool,

    #[clap(short = 'H', long, default_value_t = true)]
    pub hidden: bool,

    #[clap(long)]
    pub ignore: Vec<PathBuf>,
}

#[derive(Debug, ValueEnum, Clone, PartialEq, Eq)]
pub enum NewLine {
    LF,
    CRLF,
}
