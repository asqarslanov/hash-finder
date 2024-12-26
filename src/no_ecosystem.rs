use std::num::NonZeroUsize;
use std::sync::Arc;
use std::thread;

mod sha;
mod thread_pool;

/// This implementation only leverages the standard library.
/// The SHA-256 hashing and thread management algorithms are written manually.
///
/// Although it generally performs better than the other implementation that
/// makes use of external crates, its output order tends to be more chaotic.
#[allow(clippy::missing_panics_doc)]
pub fn find(zeros: usize) -> impl Iterator<Item = (u32, String)> {
    // The idea is that each thread will check this number of hashes.
    const ITERS_PER_THREAD: u32 = 5_000_000;
    // We need to limit the number of threads to avoid a possible overflow.
    const TOTAL_THREADS: u32 = u32::MAX / ITERS_PER_THREAD;

    let concurrent_threads = thread::available_parallelism()
        .unwrap_or(NonZeroUsize::new(4).expect("4 isn't equal to zero"));
    let pool = thread_pool::Collecting::new(concurrent_threads);

    let n_zeros = Arc::from([0].repeat(zeros));

    for thread_id in 0..TOTAL_THREADS {
        let n_zeros = Arc::clone(&n_zeros);

        pool.execute(move |collect| {
            let first_number = thread_id * ITERS_PER_THREAD + 1;
            let last_number = first_number + ITERS_PER_THREAD;

            for number in first_number..last_number {
                let hash = sha::digest(number.to_string().as_bytes());
                if hash.ends_with(&n_zeros) {
                    collect((number, sha::format(&hash)));
                }
            }
        });
    }

    // I can return the pool because it implements the `Iterator` trait.
    pool
}
