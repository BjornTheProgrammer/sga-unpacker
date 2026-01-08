use anyhow::Result;
use sga::extract_all;

use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Input file path
    input: PathBuf,

    /// Output folder path
    #[arg(short, long, value_name = "FILE")]
    output: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    extract_all(cli.input, cli.output)?;


    Ok(())
}
