use byte_unit::{Byte, UnitType};
use clap::Parser;
use std::path::PathBuf;
use std::{fs, io};

#[derive(Debug, Parser)]
#[clap(disable_help_flag = true)]
struct Opts {
    /// Directory to start from (default is current directory)
    pub dir: PathBuf,
    /// Short size in human readable format
    #[clap(short, long, default_value = "false")]
    pub human_readable: bool,
    /// Summarize disk usages
    #[clap(short, long, default_value = "false")]
    pub summarize: bool,
    /// Count links
    #[clap(short = 'l', long, default_value = "false")]
    pub count_links: bool,
    /// Show help information
    #[clap(long, short = None, action = clap::ArgAction::Help)]
    pub help: Option<bool>,
}

fn calc_disk_usage(path: PathBuf) -> io::Result<u64> {
    let mut paths = vec![path.clone()];
    let mut total = 0;
    while let Some(path) = paths.pop() {
        if path.ends_with("..") {
            continue;
        }
        // Meta data for files without following symlinks
        let meta = fs::symlink_metadata(&path)?;
        let file_type = meta.file_type();
        if file_type.is_symlink() {
            continue;
        } else if file_type.is_file() {
            // println!("{}: {}", path.display(), meta.len());
            total += meta.len();
        } else if file_type.is_dir() {
            for entry in fs::read_dir(path)? {
                if let Ok(entry) = entry {
                    paths.push(entry.path());
                }
            }
        }
    }
    Ok(total)
}

// Benchmark with hyperfine (cargo install hyperfine)
// hyperfine -L exe target/release/rdu,du '{exe} -hsl ~/SRC/linux'
fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    let start_dir = opts.dir;
    let usage_bytes = calc_disk_usage(start_dir.clone())?;
    let usage_unit = Byte::from_u64(usage_bytes).get_appropriate_unit(UnitType::Decimal);
    let usage_human = if opts.human_readable {
        format!("{usage_unit:#.1}")
    } else {
        format!("{}", usage_bytes)
    };
    println!("{}\t{}", usage_human, start_dir.display());

    Ok(())
}
