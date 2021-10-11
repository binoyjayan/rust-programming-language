
use std::time;
use std::thread;

#[allow(dead_code)]
pub fn concurrency() {

    let handle = thread::spawn(|| {
        for _ in 1..10 {
            print!("+");
            thread::sleep(time::Duration::from_millis(500))
        }
    });

    for _ in 1..10 {
        print!("-");
        thread::sleep(time::Duration::from_millis(300))
    }

    handle.join().unwrap();
}
