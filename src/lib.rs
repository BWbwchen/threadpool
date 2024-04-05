//! ThreadPool
//! A ultra simple and lightweight thread pool implementation.
#[deny(missing_docs)]

/// Thread Pool object.
pub struct ThreadPool;

impl ThreadPool {
    /// Create a new ThreadPool with maximum `nthreads` threads.
    /// # Panics
    ///
    /// if the `nthreads` is zero, it will panic.
    pub fn new(nthreads: usize) -> Self {
        assert!(nthreads > 0);
        ThreadPool {}
    }

    pub fn spawn<F>(&self, f: F)
    where
        F: FnOnce() // closure
            + Send // safely pass the &mut closure between threads.
            + 'static, // we don't know how much time that this closure will execute.
    {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_panic_new() {
        let zero_size_thread_pool = ThreadPool::new(0);
    }
}
