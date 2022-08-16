use clap::Args;

#[derive(Debug, Args)]
pub struct NewArgs {
    name: String,
    #[clap(short, long, value_parser)]
    dir_name: Option<String>,
}

impl NewArgs {
    pub fn handle(self) -> Result<(), Box<dyn std::error::Error>> {
        let h = crate::adr::handler::new::Handler{};
        h.handle(self.dir_name, &self.name)
    }
}
