use clap::Parser;

use crate::command::Cli;

mod command;
mod eolify;

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    eolify::eolify(&args)?;

    Ok(())
}
