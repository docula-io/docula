use clap::Args;
use std::error::Error;
use chrono::Utc;

#[derive(Debug, Args)]
pub struct NewCmd {
    name: String,
    #[clap(short, long, value_parser)]
    dir_name: Option<String>,
}

impl NewCmd {
    pub fn new(name: &str, dir: &str) -> NewCmd {
        NewCmd{
            name: name.to_owned(),
            dir_name: Some(dir.to_owned()),
        }
    }

    pub fn handle(self) -> Result<(), Box<dyn Error>> {
        let state = crate::state::State::load()?.adr;

        if state.dirs.is_empty() {
            Err("please set up an adr dir using the `init` command")?;
        }

        let dir = state.get_dir(&self.dir_name)?;

        match dir {
            None => Err("no adr directory could be determined")?,
            Some(x) => self.create_adr(x),
        }
    }

    fn create_adr(&self, dir: &super::Directory) -> Result<(), Box<dyn Error>> {
        let date = Utc::now().date();
        dir.create_adr(
            &self.name, date, super::Status::Proposed, 
            "",
            "",
            "",
        )
    }
}
