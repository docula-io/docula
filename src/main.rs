use clap::Parser;
use std::error::Error;
use docula;

fn main() -> Result<(), Box<dyn Error>> {
    let args = docula::Cli::parse();
    args.handle()
}
