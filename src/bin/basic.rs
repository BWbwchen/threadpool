use threadpool::ThreadPool;

fn main() {
    let max_thread = 4;
    let tp = ThreadPool::new(max_thread);
    for id in 0..max_thread {
        tp.spawn(move || {
            println!("Thread job {}", id);
        });
    }
}
