use clap::{Parser, Subcommand};
use std::error::Error;

pub mod adr;
pub mod markdown;
pub mod state;

#[derive(Debug, Parser)]
#[clap(name = "docula")]
#[clap(about = "An all in one toolbox for managing Documentation as Code", long_about=None)]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

impl Cli {
    pub fn handle(self) -> Result<(), Box<dyn Error>> {
        match self.command {
            Commands::Adr(x) => x.handle(),
            Commands::Markdown(x) => x.handle(),
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Adr(adr::command::Adr),
    Markdown(markdown::command::Markdown),
}
