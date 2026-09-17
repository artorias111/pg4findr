// cli.rs

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    #[arg(short, long, num_args = 1..)]
    pub reads: Vec<String>,

    #[arg(short, long, default_value_t = 1.0)]
    pub min_score: f64,
}
