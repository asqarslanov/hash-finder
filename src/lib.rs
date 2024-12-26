#[cfg(feature = "ecosystem")]
use rayon::iter::{ParallelBridge, ParallelIterator};

#[cfg(not(feature = "ecosystem"))]
use std::{fmt::Write, num::NonZeroUsize, sync::Arc, thread};
#[cfg(not(feature = "ecosystem"))]
use thread_pool::ThreadPool;

#[cfg(not(feature = "ecosystem"))]
mod sha;
#[cfg(not(feature = "ecosystem"))]
mod thread_pool;

/// This simple code solves the entire task.
/// Due to its small size, I suppose I can't
/// just use external crates and call it a day.
#[cfg(feature = "ecosystem")]
pub fn with_crates(zeros: usize) -> impl ParallelIterator<Item = (u32, String)> {
    extern crate rayon;
    extern crate sha256;

    let n_zeros = "0".repeat(zeros /* N */);

    (1_u32..)
        .par_bridge() // rayon::..::ParallelBridge
        .map(|num| (num, sha256::digest(num.to_string()) /* sha256 */))
        .filter(move |(_, hash)| hash.ends_with(&n_zeros))
}

#[cfg(not(feature = "ecosystem"))]
struct Res {
    pool: ThreadPool<(u32, String)>,
}

#[cfg(not(feature = "ecosystem"))]
impl Res {
    // The idea is that each thread will check this number of hashes.
    const ITERS_PER_THREAD: u32 = 5_000_000;

    // We need to limit the number of threads to avoid a possible overflow.
    const TOTAL_THREADS: u32 = u32::MAX / Self::ITERS_PER_THREAD;

    fn new(zeros: usize) -> Self {
        let threads_in_pool = thread::available_parallelism()
            .unwrap_or(NonZeroUsize::new(4).expect("4 isn't equal to zero"));

        let n_zeros = Arc::from([0].repeat(zeros));
        let pool = ThreadPool::new(threads_in_pool);

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

#[cfg(not(feature = "ecosystem"))]
impl Iterator for Res {
    type Item = (u32, String);

    fn next(&mut self) -> Option<Self::Item> {
        self.pool.next()
    }
}

/// This implementation only leverages the standard library.
/// The SHA-256 hashing and thread management algorithms are written manually.
///
/// Although it generally performs better than the other implementation that
/// makes use of external crates, its output order tends to be more chaotic.
#[cfg(not(feature = "ecosystem"))]
pub fn without_crates(zeros: usize) -> impl Iterator<Item = (u32, String)> {
    Res::new(zeros)
}

// fn without_crates_old(zeros: usize, results: usize) {
//     // The number of hashes found so far.
//     let found_so_far_global = Arc::new(Mutex::new(0));

//     for thread_id in 0..=TOTAL_THREADS {
//         let n_zeros = Arc::clone(&n_zeros_global);
//         let found_so_far = Arc::clone(&found_so_far_global);

//         pool.execute(move || {
//             let start = ITERS_PER_THREAD * thread_id + 1;
//             let end = start + ITERS_PER_THREAD;
//             for number in start..end {
//                 {
//                     let found = found_so_far
//                         .lock()
//                         .expect("locking a mutex shouldn't panic");
//                     if *found >= results {
//                         return false;
//                     }
//                 }

//                 let hash = sha::digest(number.to_string().as_bytes());
//                 if !hash.ends_with(&n_zeros) {
//                     continue;
//                 }

//                 let cmp = {
//                     let mut found = found_so_far
//                         .lock()
//                         .expect("locking a mutex shouldn't panic");
//                     *found += 1;
//                     (*found).cmp(&results)
//                 };
//                 match cmp {
//                     Ordering::Less => {
//                         print(number, hash);
//                     }
//                     Ordering::Equal => {
//                         print(number, hash);
//                         return false;
//                     }
//                     Ordering::Greater => {
//                         return false;
//                     }
//                 }
//             }

//             // This return value signifies that the thread should accept new closures.
//             true
//         });

//         let found = found_so_far_global
//             .lock()
//             .expect("locking a mutex shouldn't panic");

//         if *found >= results {
//             break;
//         }
//     }

//     pool.join();
// }
