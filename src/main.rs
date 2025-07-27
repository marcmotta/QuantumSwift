// src/main.rs
/*
 * Main executable for QuantumSwift
 */

use clap::Parser;
use quantumswift::{Result, run};

#[derive(Parser)]
#[command(version, about = "QuantumSwift - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
