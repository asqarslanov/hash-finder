use std::num::NonZeroUsize;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Job<T> = Box<dyn FnOnce(Arc<Mutex<Vec<T>>>) + Send + 'static>;

/// A simple thread pool inspired by The Book&CloseCurlyQuote;s
/// [Chapter 20.2](https://doc.rust-lang.org/book/ch20-02-multithreaded.html).
pub struct ThreadPool<T> {
    tx: mpsc::Sender<Job<T>>,
    items: Arc<Mutex<Vec<T>>>,
}

impl<T> ThreadPool<T>
where
    Mutex<Vec<T>>: Sync,
    Arc<Mutex<Vec<T>>>: Send,
    T: 'static,
{
    /// Creates a new thread pool that manages exactly `size` threads.
    pub fn new(size: NonZeroUsize) -> Self {
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));

        let items = Arc::new(Mutex::new(Vec::new()));

        for _ in 0..size.get() {
            let items = Arc::clone(&items);
            let rx = Arc::clone(&rx);

            thread::spawn(move || loop {
                let job: Job<T> = rx
                    .lock()
                    .expect("locking a mutex shouldn't panic")
                    .recv()
                    .expect("receiving from a channel shouldn't panic");

                job(Arc::clone(&items));
            });
        }

        Self { tx, items }
    }

    /// Sends a new closure to the pool.
    ///
    /// This method doesn&CloseCurlyQuote;t block the current thread.
    pub fn execute(&self, f: impl FnOnce(Arc<Mutex<Vec<T>>>) + Send + 'static) {
        let job = Box::new(f);

        self.tx
            .send(job)
            .expect("sending on a channel shouldn't panic");
    }
}

impl<T> Iterator for ThreadPool<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let item = self.items.lock().unwrap().pop();
            if item.is_some() {
                return item;
            }
        }
    }
}
