use clap::Args;
use std::error::Error;

#[derive(Debug, Args)]
pub struct NewCmd {
    name: String,
    #[clap(short, long, value_parser)]
    dir_name: Option<String>,
}

impl NewCmd {
    pub fn handle(self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
