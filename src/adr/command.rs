use clap::{Args, Subcommand};
use std::error::Error;
use super::{new, init, list};

#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct Adr {
    #[clap(subcommand)]
    command: Command,
}

impl Adr {
    pub fn handle(self) -> Result<(), Box<dyn Error>> {
        match self.command {
            Command::New(x) => x.handle(),
            Command::Init(x) => x.handle(init::Handler{}),
            Command::List(x) => x.handle(),
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum Command {
    New(new::NewCmd),
    Init(init::InitCmd),
    List(list::ListArgs),
}
