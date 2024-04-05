//! ThreadPool
//! A ultra simple and lightweight thread pool implementation.

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
pub struct ThreadPool {
    job_sender: Sender<JobType>,
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
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            job_sender,
            workers,
        }
    }

    pub fn spawn<F>(&self, f: F)
    where
        F: FnOnce() -> ThreadJobType // closure
            + Send // safely pass the &mut closure between threads.
            + 'static, // we don't know how much time that this closure will execute.
    {
        self.job_sender.send(Box::new(f)).unwrap();
    }
}

// Worker thread
struct Worker {
    id: usize,
    thread: JoinHandle<ThreadJobType>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<JobType>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Thread {} is doing jobs...", &id);
            job();
        });
        Worker { id, thread }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_panic_new() {
        let _ = ThreadPool::new(0);
    }
}
