// Disk usage calculator using async/await concurrently
use byte_unit::{Byte, UnitType};
use clap::Parser;
use futures::{stream::FuturesUnordered, StreamExt};
use lazy_static::lazy_static;
use std::{env, fs::Metadata, io, path::PathBuf};
use tokio::{fs, select};
use tokio_stream::wrappers::ReadDirStream;

use rdu::Opts;

// Maximum number of open files at a time so
// we don't run out of file descriptors
const MAX_OPEN_FILES_DEFAULT: usize = 4096;
lazy_static! {
    static ref MAX_OPEN_FILES: usize = env::var("MAX_OPEN_FILES")
        .ok()
        .and_then(|val| val.parse::<usize>().ok())
        .unwrap_or(MAX_OPEN_FILES_DEFAULT);
}

async fn meta_for_path(path: PathBuf) -> io::Result<(PathBuf, Metadata)> {
    let meta = fs::symlink_metadata(&path).await?;
    Ok((path, meta))
}

async fn calc_disk_usage(path: PathBuf) -> Result<u64, io::Error> {
    let mut meta_queue = FuturesUnordered::new();
    let mut entry_queue = FuturesUnordered::new();
    meta_queue.push(meta_for_path(path));

    let mut total = 0;
    let mut open_files: usize = 0;
    loop {
        select! {
            resolved = meta_queue.select_next_some(), if !meta_queue.is_empty() && open_files < *MAX_OPEN_FILES  => {
                let (path, meta) = resolved?;
                let file_type = meta.file_type();

                if file_type.is_symlink() {
                    // don't follow symlinks
                    continue;
                } else if file_type.is_file() {
                    total += meta.len();
                } else if file_type.is_dir() {
                    let entries = fs::read_dir(&path).await?;
                    let entry_stream = ReadDirStream::new(entries);
                    entry_queue.push(entry_stream.into_future());
                    open_files += 1;
                }
            },
            (entry, tail) = entry_queue.select_next_some(), if !entry_queue.is_empty() => {
                if let Some(Ok(entry)) = entry {
                    entry_queue.push(tail.into_future());
                    meta_queue.push(meta_for_path(entry.path()));
                } else {
                    open_files -= 1;
                }
            }
            else => break,
        }
    }

    Ok(total)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    let start_dir = opts.dir;
    let usage_bytes = calc_disk_usage(start_dir.clone()).await?;
    let usage_unit = Byte::from_u64(usage_bytes).get_appropriate_unit(UnitType::Decimal);
    let usage_human = if opts.human_readable {
        format!("{usage_unit:#.1}")
    } else {
        format!("{}", usage_bytes)
    };
    println!("{}\t{}", usage_human, start_dir.display());

    Ok(())
}
