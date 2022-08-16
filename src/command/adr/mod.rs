use clap::{Args, Subcommand};
use std::error::Error;

mod init;
mod list;
mod new;

#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct Adr {
    #[clap(subcommand)]
    command: Command,
}

impl Adr {
    pub fn handle(self) -> Result<(), Box<dyn Error>> {
        match self.command {
            Command::Init(x) => x.handle(),
            Command::New(x) => x.handle(),
            Command::List(x) => x.handle(),
        }
    }
}

#[derive(Debug, Subcommand)]
enum Command {
    #[clap(help = "Creates a new ADR in the respective directory")]
    New(new::NewArgs),
    #[clap(help = "Initializes a directory for use with ADRs")]
    Init(init::InitArgs),
    #[clap(help = "Lists all ADRs in a directory")]
    List(list::ListArgs),
}
