use std::{thread::sleep, time::Duration};

use threadpool::ThreadPool;

fn main() {
    let max_thread = 4;
    let tp = ThreadPool::new(max_thread);
    for id in 0..max_thread {
        tp.spawn(move || {
            sleep(Duration::from_millis(500));
            println!("Thread job {}", id);
        });
    }
}
