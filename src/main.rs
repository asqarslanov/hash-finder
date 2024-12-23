use clap::Parser;

use cli::Cli;

mod cli;

fn main() {
    let args = Cli::parse();

    #[cfg(feature = "ecosystem")]
    hash_finder::with_crates(args.zeros, args.results);

    #[cfg(not(feature = "ecosystem"))]
    hash_finder::without_crates(args.zeros, args.results);
}
