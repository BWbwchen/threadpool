//! # ThreadPool
//! An ultra simple thread pool implementation in Rust.

use log::{info, trace};
use std::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};
#[deny(missing_docs)]

type ThreadJobType = ();
type JobType = Box<
    dyn FnOnce() -> ThreadJobType // closure
        + Send // safely pass the &mut closure between threads.
        + 'static, // we don't know how much time that this closure will execute.
>;

/// Thread Pool object.
///
/// Example:
/// ```rust
/// use threadpool::ThreadPool;
/// # fn main() {
/// let max_thread = 4;
/// let tp = ThreadPool::new(max_thread);
/// for id in 0..max_thread {
///     tp.spawn(move || {
///         println!("Thread {}", id);
///     });
/// }
/// # }
/// ```
pub struct ThreadPool {
    // Use `Option`, since we need to take the ownership when drop.
    job_sender: Option<Sender<JobType>>,
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// Create a new ThreadPool with maximum `nthreads` threads.
    /// # Panics
    ///
    /// if the `nthreads` is zero, it will panic.
    pub fn new(nthreads: usize) -> Self {
        assert!(nthreads > 0);
        let mut workers = Vec::with_capacity(nthreads);

        let (job_sender, receiver) = channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..nthreads {
            workers.push(Worker::new(id, receiver.clone()));
        }

        ThreadPool {
            job_sender: Some(job_sender),
            workers,
        }
    }

    /// Add job into the thread pool.
    pub fn spawn<F>(&self, f: F)
    where
        F: FnOnce() -> ThreadJobType // closure
            + Send // safely pass the &mut closure between threads.
            + 'static, // we don't know how much time that this closure will execute.
    {
        self.job_sender.as_ref().unwrap().send(Box::new(f)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        trace!("Wait for the unfinished threads to finish their jobs");
        drop(self.job_sender.take());
        for worker in &mut self.workers {
            worker.thread.take().unwrap().join().unwrap();
            trace!("dropped worker {}", worker.id);
        }
    }
}

// Worker thread
struct Worker {
    id: usize,
    // Use `Option`, due to we need to take the ownership when drop.
    thread: Option<JoinHandle<ThreadJobType>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<JobType>>>) -> Self {
        let thread = thread::spawn(move || loop {
            // NOTE: use `job` variable and then match it.
            // Since at the end of the below line, the mutex will drop the lock automatically.
            // If we doesn't use a variable, the mutex will remain lock and the other thread can not grab the job.
            let job = receiver.lock().unwrap().recv();
            match job {
                Ok(job) => {
                    info!("Worker {} is doing jobs...", &id);
                    job();
                }
                _ => {
                    // A signal that sender is closed.
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use super::*;

    #[test]
    #[should_panic]
    fn test_panic_new() {
        let _ = ThreadPool::new(0);
    }

    #[test]
    fn functional_test() {
        let max_thread = 4;
        let tp = ThreadPool::new(max_thread);
        let counter = Arc::new(Mutex::new(0));

        // main thread wait for a large amount of time in order to wait for all thread to perform the job.
        let thread_sleep_time = 50;
        let main_sleep_time = thread_sleep_time * max_thread as u64;

        for _ in 0..max_thread {
            let c = counter.clone();
            tp.spawn(move || {
                sleep(Duration::from_millis(thread_sleep_time));
                let mut cc = c.lock().unwrap();
                *cc += 1;
            });
        }

        sleep(Duration::from_millis(main_sleep_time));
        assert_eq!(*counter.lock().unwrap(), max_thread);
    }
    #[test]
    fn drop_test() {
        env_logger::builder()
            .filter_level(log::LevelFilter::Trace)
            .init();
        let max_thread = 4;
        let tp = ThreadPool::new(max_thread);
        let counter = Arc::new(Mutex::new(0));

        let thread_sleep_time = 50;

        for _ in 0..max_thread {
            let c = counter.clone();
            tp.spawn(move || {
                sleep(Duration::from_millis(thread_sleep_time));
                let mut cc = c.lock().unwrap();
                *cc += 1;
            });
        }

        // main thread will NOT wait for all thread, drop the thread pool directly.
        // It should drop all the working thread.
        // The threadpool should finishes all the unfinished job and close the thread pool gracefully.
        drop(tp);

        assert_eq!(
            *counter.lock().unwrap(),
            max_thread,
            "Thread Pool should close gracefully"
        );
    }
}
