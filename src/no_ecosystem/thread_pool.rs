use std::num::NonZeroUsize;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type CollectFn<T> = Box<dyn Fn(T)>;
type Job<T> = Box<dyn FnOnce(CollectFn<T>) + Send>;

/// A simple thread pool inspired by The Book&CloseCurlyQuote;s
/// [Chapter 20.2](https://doc.rust-lang.org/book/ch20-02-multithreaded.html).
pub struct Collecting<T: Send + 'static> {
    tx: mpsc::Sender<Job<T>>,
    items: Arc<Mutex<Vec<T>>>,
}

impl<T: Send + 'static> Collecting<T> {
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
                    .expect("locking a mutex shouldn't fail")
                    .recv()
                    .expect("receiving from a channel shouldn't fail");

                let items = Arc::clone(&items);
                let collect = Box::new(move |value| {
                    items
                        .clone()
                        .lock()
                        .expect("locking a mutex shouldn't fail")
                        .push(value);
                });

                job(collect);
            });
        }

        Self { tx, items }
    }

    /// Sends a new closure to the pool.
    ///
    /// This method doesn&CloseCurlyQuote;t block the current thread.
    pub fn execute(&self, f: impl FnOnce(CollectFn<T>) + Send + 'static) {
        let job = Box::new(f);

        self.tx
            .send(job)
            .expect("sending on a channel shouldn't fail");
    }
}

impl<T: Send + 'static> Iterator for Collecting<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let item = self
                .items
                .lock()
                .expect("unlocking a mutex shouldn't fail")
                .pop();

            if item.is_some() {
                return item;
            }
        }
    }
}
