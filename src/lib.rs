pub mod sha;

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

#[cfg(not(feature = "ecosystem"))]
pub fn without_crates(zeros: usize, results: usize) {
    let n_zeros = [0].repeat(zeros /* N */);

    (1_u32..)
        .map(|num| (num, sha::digest(num.to_string().as_bytes())))
        .filter(|(_, hash)| hash.ends_with(&n_zeros))
        .take(results /* F */)
        .for_each(|(number, hash)| {
            let hash_formatted = hash
                .chunks(2)
                .map(|it| format!("{:02x}", 16 * it[0] + it[1]))
                .collect::<Box<str>>();

            println!(r#"{number}, "{hash_formatted}""#);
        });
}
