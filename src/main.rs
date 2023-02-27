mod file;

use colored::{Color, Colorize};
use std::io;
use std::path::PathBuf;

use clap::Parser;

const COLOR_COUNT: usize = 5;
const OVERVIEW_LIMIT: usize = 5;

const COLORS: [Color; COLOR_COUNT] = [
    Color::TrueColor {
        r: 254,
        g: 74,
        b: 73,
    },
    Color::TrueColor {
        r: 42,
        g: 183,
        b: 202,
    },
    Color::TrueColor {
        r: 254,
        g: 215,
        b: 102,
    },
    Color::TrueColor {
        r: 230,
        g: 230,
        b: 234,
    },
    Color::TrueColor {
        r: 244,
        g: 244,
        b: 248,
    },
];

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The path to print the file tree for
    path: PathBuf,

    /// How deep to recurse
    #[arg(short, long, default_value_t = 5)]
    level: u32,

    /// Include files starting with a '.'
    #[arg(short, long, default_value_t = false)]
    all: bool,

    /// Only print directories
    #[arg(short, long, default_value_t = false)]
    directory: bool,

    /// Print the full path prefix for each file
    #[arg(short, long, default_value_t = false)]
    full: bool,

    /// Disables coloring
    #[arg(long, default_value_t = false)]
    no_color: bool,

    /// Disables icons
    #[arg(long, default_value_t = false)]
    no_icons: bool,

    #[arg(long, default_value_t = false)]
    overview: bool,
}

#[derive(Debug)]
enum ErrReason {
    PathInvalid(String),
}

fn main() {
    let cli = Cli::parse();

    // Todo: Handle errors here

    match validate(&cli) {
        Ok(_) => {
            tree(&cli).unwrap();
        }
        Err(e) => println!("Error! {:?}", e),
    }
}

fn validate(cli: &Cli) -> Result<(), ErrReason> {
    let path = &cli.path;
    if !path.is_dir() {
        return Err(ErrReason::PathInvalid(String::from(path.to_string_lossy())));
    }

    Ok(())
}

fn tree(cli: &Cli) -> io::Result<String> {
    let mut out = String::new();
    let mut info = FileInfo {
        directories: 0,
        files: 0,
    };

    #[cfg(test)]
    out.push_str(format!("{}\n", cli.path.to_string_lossy()).as_str());
    #[cfg(not(test))]
    print!("{}\n", cli.path.to_string_lossy());

    tree_recurse(&cli.path, cli, &mut out, "", 0, &mut info)?;

    #[cfg(test)]
    out.push_str(format!("{} directories and {} files", info.directories, info.files).as_str());
    #[cfg(not(test))]
    print!("{} directories and {} files", info.directories, info.files);

    Ok(out)
}

struct FileInfo {
    directories: u32,
    files: u32,
}

fn tree_recurse(
    path: &PathBuf,
    cli: &Cli,
    out: &mut String,
    prefix: &str,
    depth: u32,
    info: &mut FileInfo,
) -> io::Result<()> {
    if depth > cli.level {
        return Ok(());
    }

    // Extract the paths from all the correct dir entries
    let path_iter = path.read_dir()?.filter_map(|e| e.ok()).map(|e| e.path());

    // Filter out unwanted files based on cli options
    let mut paths = path_iter
        .filter(|p| !ignore(p, cli))
        .collect::<Vec<PathBuf>>();

    paths.sort_by(|a, b| a.file_name().unwrap().cmp(b.file_name().unwrap()));

    let mut it = paths.iter().peekable();
    let mut i = 0;
    while let Some(p) = it.next() {
        let last = it.peek().is_none();

        let file = file::File::from(p);

        if !cli.overview || i < OVERVIEW_LIMIT {
            #[cfg(test)]
            out.push_str(&get_line(&file, prefix, cli, last));
            #[cfg(not(test))]
            print!("{}", get_line(&file, prefix, cli, last));
        } else if cli.overview && i == OVERVIEW_LIMIT {
            #[cfg(test)]
            out.push_str(format!("{}    {}\n", prefix, "...").as_str());
            #[cfg(not(test))]
            print!("{}    {}\n", prefix, "...");
        }

        if p.is_dir() {
            info.directories += 1;
            let new_prefix = format!("{}{}", prefix, if last { "    " } else { "│   " });
            tree_recurse(&p, cli, out, &new_prefix, depth + 1, info)?;
        } else {
            info.files += 1;
        }
        i += 1;
    }

    Ok(())
}

fn ignore(path: &PathBuf, cli: &Cli) -> bool {
    !cli.all && path.file_name().unwrap().to_string_lossy().starts_with('.')
        || cli.directory && !path.is_dir()
}

fn get_line(file: &file::File, prefix: &str, cli: &Cli, last: bool) -> String {
    // let colored_str = if (depth as usize) < COLOR_COUNT {
    //     filename.color(COLORS[depth as usize])
    // } else {
    //     filename.white()
    // };

    format!(
        "{}{} {}\n",
        prefix,
        if last { "└──" } else { "├──" },
        file.to_string(cli),
        // if cli.no_color {
        //     filename.to_owned().to_string()
        // } else {
        //     colored_str.to_string()
        // }
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq; // crate for test-only use. Cannot be used in non-test code.
    use std::path::PathBuf;

    fn get_default() -> Cli {
        Cli {
            path: PathBuf::from("test"),
            level: 5,
            all: false,
            directory: false,
            full: false,
            no_color: true,
            no_icons: true,
            overview: true,
        }
    }

    #[test]
    fn default_test() -> io::Result<()> {
        let cli = get_default();
        let expected = "test
├── a
│   └── b
├── c.txt
├── d
│   ├── e
│   │   └── f
│   │       └── g
│   └── h.txt
└── d.txt
6 directories and 3 files";
        assert_eq!(tree(&cli)?, expected);
        Ok(())
    }

    #[test]
    fn directory_test() -> io::Result<()> {
        let mut cli = get_default();
        cli.directory = true;

        let expected = "test
├── a
│   └── b
└── d
    └── e
        └── f
            └── g
6 directories and 0 files";
        assert_eq!(tree(&cli)?, expected);
        Ok(())
    }

    #[test]
    fn all_test() -> io::Result<()> {
        let mut cli = get_default();
        cli.all = true;

        let expected = "test
├── a
│   └── b
│       └── .gitkeep
├── c.txt
├── d
│   ├── e
│   │   ├── .gitkeep
│   │   └── f
│   │       ├── .gitkeep
│   │       └── g
│   │           └── .gitkeep
│   └── h.txt
└── d.txt
6 directories and 7 files";

        assert_eq!(tree(&cli)?, expected);
        Ok(())
    }

    #[test]
    fn full_test() -> io::Result<()> {
        let mut cli = get_default();
        cli.full = true;

        let expected = "test
├── test/a
│   └── test/a/b
├── test/c.txt
├── test/d
│   ├── test/d/e
│   │   └── test/d/e/f
│   │       └── test/d/e/f/g
│   └── test/d/h.txt
└── test/d.txt
6 directories and 3 files";

        assert_eq!(tree(&cli)?, expected);
        Ok(())
    }
} /* tests */
