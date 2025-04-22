//! This example demonstrates how to spawn multiple threads
//! and wait for them to finish in the same order they were spawned.

use anyhow::{Context, Result};

pub fn threading(num_items: usize, num_workers: usize) -> Result<()> {
    let (tx, rx) = crossbeam_channel::bounded(num_workers);

    let producer = std::thread::spawn(move || {
        for i in 1..=num_items {
            // spawn a new thread
            let handle = std::thread::spawn(move || i * 10);
            tx.send(handle).context("Failed to send handle").unwrap();
        }
    });


    let consumer = std::thread::spawn(move || {
        println!("Waiting for consumers to finish...");
        // sleep for a while for results to be ready
        std::thread::sleep(std::time::Duration::from_secs(1));
        while let Ok(handle) = rx.recv() {
            let result = handle.join().expect("worker thread failed");
            println!("Result [Thread spawn]: {}", result);
        }
    });

    producer.join().expect("producer thread failed");
    consumer.join().expect("consumer thread failed");

    Ok(())
}
