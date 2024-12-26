use clap::Parser;

use cli::Cli;

mod cli;

fn main() {
    let args = Cli::parse();

    #[cfg(feature = "ecosystem")]
    {
        use rayon::iter::ParallelIterator;

        hash_finder::with_crates(args.zeros)
            .take_any(args.results)
            .for_each(|(number, hash)| {
                println!("{}", format(number, &hash));
            });
    }

    #[cfg(not(feature = "ecosystem"))]
    {
        hash_finder::without_crates(args.zeros)
            .take(args.results)
            .for_each(|(number, hash)| {
                println!("{}", format(number, &hash));
            });
    }
}

fn format(number: u32, hash: &str) -> String {
    format!(r#"{number}, "{hash}""#)
}
