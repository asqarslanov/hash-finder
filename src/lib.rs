use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex};
use std::thread;

use thread_pool::ThreadPool;

mod sha;
mod thread_pool;

/// This simple code solves the entire task.
/// Due to its small size, I suppose I can't
/// just use external crates and call it a day.
#[cfg(feature = "ecosystem")]
pub fn with_crates(zeros: usize, results: usize) {
    extern crate rayon;
    extern crate sha256;

    use rayon::iter::{ParallelBridge, ParallelIterator};

    let n_zeros = "0".repeat(zeros /* N */);

    (1_u32..)
        .par_bridge() // rayon::..::ParallelBridge
        .map(|num| (num, sha256::digest(num.to_string()) /* sha256 */))
        .filter(|(_, hash)| hash.ends_with(&n_zeros))
        .take_any(results /* F */)
        .for_each(|(number, hash)| {
            println!(r#"{number}, "{hash}""#);
        });
}

fn print(number: u32, hash: [u8; 64]) {
    let hash_formatted = hash
        .chunks(2)
        .map(|it| format!("{:02x}", 16 * it[0] + it[1]))
        .collect::<Box<str>>();

    println!(r#"{number}, "{hash_formatted}""#);
}

#[allow(clippy::missing_panics_doc)]
#[cfg(not(feature = "ecosystem"))]
pub fn without_crates(zeros: usize, results: usize) {
    use std::cmp::Ordering;

    const ITERS_PER_THREAD: u32 = 5_000_000;
    const TOTAL_THREADS: u32 = u32::MAX / ITERS_PER_THREAD;

    let threads = thread::available_parallelism()
        .unwrap_or(NonZeroUsize::new(4).expect("4 isn't equal to zero"));

    let pool = ThreadPool::new(threads);

    let n_zeros = Arc::from([0].repeat(zeros /* N */));
    let global_found = Arc::new(Mutex::new(0));

    for thread_id in 0..=TOTAL_THREADS {
        let n_zeros = Arc::clone(&n_zeros);
        let found = Arc::clone(&global_found);

        pool.execute(move || {
            let start = ITERS_PER_THREAD * thread_id + 1;
            let end = start + ITERS_PER_THREAD;
            for number in start..end {
                {
                    let found = found.lock().expect("locking a mutex shouldn't panic");
                    if *found >= results {
                        return false;
                    }
                }

                let hash = sha::digest(number.to_string().as_bytes());
                if !hash.ends_with(&n_zeros) {
                    continue;
                }

                let cmp = {
                    let mut found = found.lock().expect("locking a mutex shouldn't panic");
                    *found += 1;
                    (*found).cmp(&results)
                };

                match cmp {
                    Ordering::Less => {
                        print(number, hash);
                    }
                    Ordering::Equal => {
                        print(number, hash);
                        return false;
                    }
                    Ordering::Greater => {
                        return false;
                    }
                }
            }

            true
        });

        let found_tmp = global_found
            .lock()
            .expect("locking a mutex shouldn't panic");

        if *found_tmp >= results {
            break;
        }
    }

    pool.join();
}
