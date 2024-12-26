use std::fmt::Write;
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::thread;

use thread_pool::CollectingThreadPool;

mod sha;
mod thread_pool;

/// This implementation only leverages the standard library.
/// The SHA-256 hashing and thread management algorithms are written manually.
///
/// Although it generally performs better than the other implementation that
/// makes use of external crates, its output order tends to be more chaotic.
pub fn find(zeros: usize) -> impl Iterator<Item = (u32, String)> {
    NumHashIter::new(zeros)
}

/// This simple code solves the entire task.
/// Due to its small size, I suppose I can't
/// just use external crates and call it a day.
struct NumHashIter {
    pool: CollectingThreadPool<(u32, String)>,
}

impl NumHashIter {
    // The idea is that each thread will check this number of hashes.
    const ITERS_PER_THREAD: u32 = 5_000_000;

    // We need to limit the number of threads to avoid a possible overflow.
    const TOTAL_THREADS: u32 = u32::MAX / Self::ITERS_PER_THREAD;

    fn new(zeros: usize) -> Self {
        let threads_in_pool = thread::available_parallelism()
            .unwrap_or(NonZeroUsize::new(4).expect("4 isn't equal to zero"));

        let n_zeros = Arc::from([0].repeat(zeros));
        let pool = CollectingThreadPool::new(threads_in_pool);

        for thread_id in 0..=Self::TOTAL_THREADS {
            let n_zeros = Arc::clone(&n_zeros);

            pool.execute(move |items| {
                let start = thread_id * Self::ITERS_PER_THREAD + 1;
                let end = start + Self::ITERS_PER_THREAD;
                for number in start..end {
                    let hash = sha::digest(number.to_string().as_bytes());
                    if hash.ends_with(&n_zeros) {
                        let hash_formatted =
                            hash.chunks(2).fold(String::new(), |mut output, it| {
                                write!(output, "{:02x}", 16 * it[0] + it[1]).unwrap();
                                output
                            });
                        items.lock().unwrap().push((number, hash_formatted));
                    }
                }
            });
        }

        Self { pool }
    }
}

impl Iterator for NumHashIter {
    type Item = (u32, String);

    fn next(&mut self) -> Option<Self::Item> {
        self.pool.next()
    }
}
