use clap::Parser;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = docula::Cli::parse();
    args.handle()
}
