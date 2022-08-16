use clap::Args;

#[derive(Debug, Args)]
pub struct ListArgs {
    #[clap(short, long, value_parser, help = "The name of the ADR dir")]
    name: Option<String>,
}

impl ListArgs {
    pub fn handle(self) -> Result<(), Box<dyn std::error::Error>> {
        let h = crate::adr::handler::list::Handler{};
        h.handle(self.name)
    }
}
