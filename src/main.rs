use clap::Parser;

use cli::Cli;

mod cli;

fn main() {
    let args = Cli::parse();

    #[cfg(feature = "ecosystem")]
    {
        use rayon::iter::ParallelIterator;

        hash_finder::with_crates(args.zeros /* N */)
            .take_any(args.results /* F */)
            .for_each(|(number, hash)| {
                println!("{}", format(number, &hash));
            });
    }

    #[cfg(not(feature = "ecosystem"))]
    {
        hash_finder::without_crates(args.zeros /* N */)
            .take(args.results /* F */)
            .for_each(|(number, hash)| {
                println!("{}", format(number, &hash));
            });
    }
}

/// Formats the given number and string such as the following.
/// - `123, "abc123dfg456"`
fn format(number: u32, hash: &str) -> String {
    format!(r#"{number}, "{hash}""#)
}
