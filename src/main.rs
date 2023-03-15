use std::{
    env, fs,
    io::{self, Write},
    path::PathBuf,
};

use file::File;

mod file;

struct State {
    depth: u64,
    dir: PathBuf,
}

impl State {
    pub fn new(dir: PathBuf, depth: u64) -> Self {
        Self { dir, depth }
    }
}

fn tree<W: Write>(writer: &mut W, state: State) -> io::Result<()> {
    for (i, e) in fs::read_dir(&state.dir)?.enumerate() {
        let entry = e?;
        let path = entry.path();
        let file = File::from_path(&path)?;

        writeln!(
            writer,
            "{:<d$}{name:}",
            "",
            d = state.depth as usize,
            name = file,
        )
        .expect("Unable to write");

        if path.is_dir() {
            tree(writer, State::new(path, state.depth + 1))?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_test() {
        let mut output = Vec::new();
        let expected = "";

        // tree(&mut output);

        let actual = String::from_utf8(output).expect("Not UTF-8");
        assert_eq!(expected, actual);
    }
}

fn main() -> io::Result<()> {
    let dir = env::current_dir()?;
    let state = State::new(dir, 0);
    tree(&mut io::stdout(), state)?;

    Ok(())
}
