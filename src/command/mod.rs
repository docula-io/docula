use clap::Subcommand;

mod adr;
mod markdown;

#[derive(Debug, Subcommand)]
pub enum Command {
    Adr(adr::Adr),
    Markdown(markdown::Markdown),
}

impl Command {
    pub fn handle(self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Command::Adr(x) => x.handle(),
            Command::Markdown(x) => x.handle(),
        }
    }
}
