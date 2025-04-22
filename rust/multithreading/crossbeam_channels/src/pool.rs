//! This example demonstrates how to use threadpool
//! that does not ensure the order of the results.

use anyhow::Result;
use threadpool::ThreadPool;

pub fn thread_pool(num_items: usize, num_workers: usize) -> Result<()> {
    let pool = ThreadPool::new(num_workers);
    let (tx, rx) = crossbeam_channel::bounded(num_workers);

    let producer = std::thread::spawn(move || {
        for i in 1..=num_items {
            let tx = tx.clone();
            pool.execute(move || {
                let result = i * 10;
                tx.send(result).unwrap();
            });
        }
        // Drop the sender to close the channel
        drop(tx);
    });
    
    let consumer = std::thread::spawn(move || {
        println!("Waiting for consumers to finish...");
        for received in rx {
            println!("Result [ThreadPool]: {}", received);
        }
    });

    producer.join().expect("producer thread failed");
    consumer.join().expect("consumer thread failed");

    Ok(())
}
