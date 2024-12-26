use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// Number of zeros
    #[arg(short = 'N', value_name = "NUMBER")]
    pub zeros: usize,

    /// Number of results
    #[arg(short = 'F', value_name = "NUMBER")]
    pub results: usize,
}
