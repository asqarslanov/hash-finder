use rayon::prelude::*;

pub fn find(zeros: usize) -> impl ParallelIterator<Item = (u32, String)> {
    let n_zeros = "0".repeat(zeros);

    (1_u32..)
        .par_bridge() // rayon::..::ParallelBridge
        .map(|num| (num, sha256::digest(num.to_string())))
        .filter(move |(_, hash)| hash.ends_with(&n_zeros))
}
