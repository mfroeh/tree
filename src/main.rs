use std::{env, io, path::PathBuf};

mod file;
mod tree;

pub struct Config {
    directory: PathBuf,
}

fn main() -> io::Result<()> {
    tree::tree(
        &mut io::stdout(),
        Config {
            directory: env::current_dir()?,
        },
    )
}
