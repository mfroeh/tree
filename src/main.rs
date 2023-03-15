use std::{
    env, io,
    path::{Path, PathBuf},
};

use clap::Parser;
use tree::tree;

mod file;
mod tree;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Config {
    #[arg(default_value = ".")]
    directory: PathBuf,

    /// Do not ignore entries starting with .
    #[arg(short, long)]
    all: bool,

    /// Only display directories ignoring all files
    #[arg(short, long)]
    directory_only: bool,

    /// The recursion depth
    #[arg(short, long, default_value_t = 3)]
    limit: u32,
}

fn main() -> io::Result<()> {
    let config = Config::parse();

    tree::tree(config)
}
