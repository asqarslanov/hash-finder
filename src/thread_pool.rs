use std::num::NonZeroUsize;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() -> bool + Send + 'static>;

/// A simple thread pool inspired by The Book&CloseCurlyQuote;s
/// [Chapter 20.2](https://doc.rust-lang.org/book/ch20-02-multithreaded.html).
pub struct ThreadPool {
    threads: Box<[thread::JoinHandle<()>]>,
    tx: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: NonZeroUsize) -> Self {
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));

        let threads = (0..size.get())
            .map(|_| {
                let rx = Arc::clone(&rx);

                thread::spawn(move || loop {
                    let job: Job = rx
                        .lock()
                        .expect("locking a mutex shouldn't panic")
                        .recv()
                        .expect("receiving from a channel shouldn't panic");

                    if !job() {
                        break;
                    }
                })
            })
            .collect::<Box<[_]>>();

        Self { threads, tx }
    }

    pub fn execute(&self, f: impl FnOnce() -> bool + Send + 'static) {
        let job = Box::new(f);

        self.tx
            .send(job)
            .expect("sending on a channel shouldn't panic");
    }

    pub fn join(self) {
        for thread in self.threads {
            thread.join().expect("joining a thread shouldn't panic");
        }
    }
}
