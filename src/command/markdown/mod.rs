mod fmt;
mod lint;

use clap::{Args, Subcommand};

#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct Markdown {
    #[clap(subcommand)]
    command: Command,
}

impl Markdown {
    pub fn handle(self) -> Result<(), Box<dyn std::error::Error>> {
        match self.command {
            Command::Fmt(x) => x.handle(),
            Command::Lint(x) => x.handle(),
        }
    }
}

#[derive(Debug, Subcommand)]
enum Command {
    Fmt(fmt::FmtArgs),
    Lint(lint::LintArgs),
}
