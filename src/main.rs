use clap::Parser;

use cli::Cli;

mod cli;

fn main() {
    let args = Cli::parse();
    hash_finder::find(args.zeros, args.results);
}
