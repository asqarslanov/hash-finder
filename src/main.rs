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
