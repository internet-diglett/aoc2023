use std::fs;

use anyhow::{anyhow, Result};
use clap::Parser;

/// Args for running the CLI program for the AoC puzzle solver
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// which day's puzzle are you solving?
    #[arg(short, long)]
    day: usize,

    /// plaintext file containing your unique puzzle input
    #[arg(short, long)]
    input: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let text = fs::read_to_string(args.input)?;
    match args.day {
        1 => day1::print_answers(&text)?,
        _ => return Err(anyhow!("Solver not implemented for day {}", args.day)),
    };
    Ok(())
}
