use std::{
    env, io,
    path::{Path, PathBuf},
};

use clap::Parser;

mod file;
mod tree;

#[derive(Parser)]
pub struct Cli {
    directory: Option<PathBuf>,
}

pub struct Config {
    dir: PathBuf,
}

impl TryFrom<Cli> for Config {
    type Error = io::Error;

    fn try_from(cli: Cli) -> Result<Self, Self::Error> {
        let dir = cli.directory.unwrap_or(PathBuf::from("."));
        Ok(Config { dir })
    }
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let config = Config::try_from(cli)?;

    tree::tree(&mut io::stdout(), config)
}
