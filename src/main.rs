// src/main.rs
/*
 * Main executable for IntelliSenseOptimizerChainPro
 */

use clap::Parser;
use intellisenseoptimizerchainpro::{Result, run};

#[derive(Parser)]
#[command(version, about = "IntelliSenseOptimizerChainPro - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
