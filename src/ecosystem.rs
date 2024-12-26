use rayon::prelude::*;

/// This implementation makes use of two external crates:
/// [`rayon`] for parallelization and [`sha256`] for calculating SHA-256 hashes.
///
/// # Examples
///
/// ```rust
/// use rayon::iter::ParallelIterator;
///
/// hash_finder::find(3)
///     .take_any(4)
///     .for_each(|(number, hash)| {
///         println!("{number}: {hash}");
///     });
pub fn find(zeros: usize) -> impl ParallelIterator<Item = (u32, String)> {
    let n_zeros = "0".repeat(zeros);

    (1_u32..)
        .par_bridge() // rayon::..::ParallelBridge
        .map(|num| (num, sha256::digest(num.to_string())))
        .filter(move |(_, hash)| hash.ends_with(&n_zeros))
}

#[cfg(test)]
mod tests {
    use std::ops::RangeInclusive;

    use super::*;

    const TEST_NS: RangeInclusive<usize> = 1..=4;
    const TEST_F: usize = 25;

    #[test]
    fn hashes_match() {
        for zeros in TEST_NS {
            find(zeros).take_any(TEST_F).for_each(|(num, hash)| {
                assert_eq!(hash, sha256::digest(num.to_string()));
            })
        }
    }

    #[test]
    fn zeros_match() {
        for zeros in TEST_NS {
            find(zeros)
                .take_any(TEST_F)
                .for_each(|(_, hash)| assert!(hash.ends_with(&"0".repeat(zeros))));
        }
    }
}
