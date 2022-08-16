use clap::Parser;
use std::error::Error;

pub mod adr;
mod command;
pub mod markdown;
pub mod state;

#[derive(Debug, Parser)]
#[clap(name = "docula")]
#[clap(about = "An all in one toolbox for managing Documentation as Code", long_about=None)]
pub struct Cli {
    #[clap(subcommand)]
    command: command::Command,
}

impl Cli {
    pub fn handle(self) -> Result<(), Box<dyn Error>> {
        self.command.handle()
    }
}
