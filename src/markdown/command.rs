use clap::{Args, Subcommand};
use super::fmt::FmtCmd;

#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct Markdown {
    #[clap(subcommand)]
    command: Command
}

impl Markdown {
    pub fn handle(self) -> Result<(), Box<dyn std::error::Error>> {
        match self.command {
            Command::Fmt(x) => x.handle(),
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Fmt(FmtCmd)
}
