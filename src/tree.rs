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

pub fn tree(config: Config) -> io::Result<()> {
    let tree = Tree::new(config);
    tree.tree(&mut io::stdout())
}

struct State<'a> {
    depth: u32,
    dir: PathBuf,
    prefix: &'a str,
}

impl<'a> State<'a> {
    pub fn new(dir: PathBuf, depth: u32, prefix: &'a str) -> Self {
        Self { dir, depth, prefix }
    }
}

struct Tree {
    config: Config,
}

impl Tree {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    fn tree<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        let state = State::new(self.config.directory.clone(), 0, "");
        self.tree_rec(writer, state)
    }

    fn tree_rec<W: Write>(&self, writer: &mut W, state: State) -> io::Result<()> {
        if state.depth == 0 {
            let file = File::from_path(&state.dir)?;
            writeln!(writer, "{}", file).expect("Unable to write");
        }

        if state.depth >= self.config.limit {
            return Ok(());
        }

        let mut entries: Vec<DirEntry> = fs::read_dir(&state.dir)?
            .filter_map(|e| if let Ok(e) = e { Some(e) } else { None })
            .filter(|e| self.entry_predicate(e, &state))
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
                self.tree_rec(writer, State::new(path, state.depth + 1, &prefix))?;
            }
        }

        Ok(())
    }

    fn entry_predicate(&self, entry: &DirEntry, state: &State) -> bool {
        if let Ok(meta) = entry.metadata() {
            if self.config.directory_only && !meta.is_dir() {
                return false;
            }

            if !self.config.all
                && entry
                    .file_name()
                    .to_str()
                    .expect("Not valid UTF-8")
                    .starts_with('.')
            {
                return false;
            }

            true
        } else {
            false
        }
    }
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
