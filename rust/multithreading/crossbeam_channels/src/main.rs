use anyhow::Result;
use std::env;

mod pool;
mod threading;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <num> <num_workers>", args[0]);
        return Ok(());
    }

    let num_items: usize = args[1].parse().unwrap();
    let num_workers: usize = args[2].parse().unwrap();

    threading::threading(num_items, num_workers)?;
    pool::thread_pool(num_items, num_workers)?;
    Ok(())
}
