use std::{
    fs::{self, DirEntry},
    io::{self, Write},
    path::PathBuf,
};

use crate::{file::File, Config};

const PREFIX: &str = "│   ";
const PREFIX_LAST: &str = "    ";
const FRONT: &str = "├──";
const FRONT_LAST: &str = "└──";

pub fn tree<W: Write>(writer: &mut W, config: Config) -> io::Result<()> {
    let dir = config.directory;
    let state = State::new(dir, 0, "");

    tree_rec(writer, state)?;
    Ok(())
}

struct State<'a> {
    depth: u64,
    dir: PathBuf,
    prefix: &'a str,
}

impl<'a> State<'a> {
    pub fn new(dir: PathBuf, depth: u64, prefix: &'a str) -> Self {
        Self { dir, depth, prefix }
    }
}

fn tree_rec<W: Write>(writer: &mut W, state: State) -> io::Result<()> {
    if state.depth == 0 {
        let file = File::from_path(&state.dir)?;
        writeln!(writer, "{}", file).expect("Unable to write");
    }

    let mut entries: Vec<DirEntry> = fs::read_dir(&state.dir)?
        .filter_map(|e| if let Ok(e) = e { Some(e) } else { None })
        .collect();
    entries.sort_by_key(|e| e.file_name());
    let dir_limit = entries.len();

    for (i, entry) in entries.into_iter().enumerate() {
        let is_last = i == (dir_limit - 1);
        let path = entry.path();
        let file = File::from_path(&path)?;

        writeln!(
            writer,
            "{}{} {name:}",
            state.prefix,
            if is_last { FRONT_LAST } else { FRONT },
            name = file,
        )
        .expect("Unable to write");

        if path.is_dir() {
            let mut prefix = String::from(state.prefix);
            prefix.push_str(if is_last { PREFIX_LAST } else { PREFIX });
            tree_rec(writer, State::new(path, state.depth + 1, &prefix))?;
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
