use clap::Parser;

use cli::Args;

mod cli;

fn main() {
    let args = Args::parse();

    #[cfg(feature = "ecosystem")]
    with_crates(args.zeros, args.results);

    #[cfg(not(feature = "ecosystem"))]
    without_crates(args.zeros, args.results);
}

#[cfg(feature = "ecosystem")]
fn with_crates(zeros: usize, results: usize) {
    use rayon::iter::ParallelIterator;

    hash_finder::find(zeros)
        .take_any(results)
        .for_each(|(number, hash)| {
            println!("{}", format(number, &hash));
        });
}

#[cfg(not(feature = "ecosystem"))]
fn without_crates(zeros: usize, results: usize) {
    hash_finder::find(zeros)
        .take(results)
        .for_each(|(number, hash)| {
            println!("{}", format(number, &hash));
        });
}

/// Formats the given number and string such as the following.
/// - `123, "abc123dfg456"`
fn format(number: u32, hash: &str) -> String {
    format!(r#"{number}, "{hash}""#)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_formatting() {
        let first = format(1, "a");
        assert_eq!(first, r#"1, "a""#);

        let second = format(123, "abc123dfg456");
        assert_eq!(second, r#"123, "abc123dfg456""#);

        let third = format(
            4163,
            "95d4362bd3cd4315d0bbe38dfa5d7fb8f0aed5f1a31d98d510907279194e3000",
        );
        assert_eq!(
            third,
            r#"4163, "95d4362bd3cd4315d0bbe38dfa5d7fb8f0aed5f1a31d98d510907279194e3000""#,
        );
    }
}
