use std::{fmt::format, path::PathBuf};

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    path: PathBuf,

    #[arg(short, long, default_value_t = 3)]
    level: u32,
}

enum ErrReason<'a> {
    PathInvalid(&'a str),
}

type TreeResult<'a> = Result<(), ErrReason<'a>>;

fn main() {
    let cli = Cli::parse();

    match validate(&cli) {
        Ok(_) => println!("{}", tree(cli)),
        Err(e) => match e {
            ErrReason::PathInvalid(s) => println!("{} is not a valid path", s),
        },
    }
}

fn validate(cli: &Cli) -> TreeResult {
    let path = &cli.path;
    if !path.is_dir() {
        return Err(ErrReason::PathInvalid(path.to_str().unwrap()));
    }

    Ok(())
}

fn tree(cli: Cli) -> String {
    let depth = cli.level;
    let path = cli.path;

    let mut out = String::new();
    tree_recurse(&mut out, path, 0, depth);
    out
}

fn tree_recurse(out: &mut String, path: PathBuf, depth: u32, limit: u32) {
    if depth > limit {
        return;
    }

    out.push_str(format!("{:->d$}\n", path.file_name().unwrap().to_string_lossy(), d = (depth * 2) as usize).as_str());

    if path.is_dir() {
        for res in path.read_dir().unwrap().into_iter() {
            if let Ok(f) = res {
                tree_recurse(out, f.path(), depth + 1, limit);
            } else {
                panic!("Failed to read_dir");
            }
        }
    }
}

mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn tree_test() {
        let cli = Cli {
            path: PathBuf::from("./test"),
            level: 10,
        };

        assert!(cli.path.exists());

        let expected = "test
├── a
│   └── b
├── c.txt
├── d
│   ├── e
│   │   └── f
│   │       └── g
│   └── h.txt
└── d.txt
";
        assert_eq!(tree(cli), expected);
    }

    #[test]
    fn tree_depth_test() {
        let cli = Cli {
            path: PathBuf::from("./test"),
            level: 2,
        };

        assert!(cli.path.exists());

        let expected = "test
├── a
│   └── b
├── c.txt
├── d
│   ├── e
│   │   └── f
│   └── h.txt
└── d.txt
";
        assert_eq!(tree(cli), expected);
    }
} /* tests */
