use chrono::Utc;
use std::error::Error;
use crate::adr::{Directory, Status};

pub struct Handler {}

impl Handler {
    pub fn handle(self, dir_name: Option<String>, name: &str) -> Result<(), Box<dyn Error>> {
        let state = crate::state::State::load()?.adr;

        if state.dirs.is_empty() {
            Err("please set up an adr dir using the `init` command")?;
        }

        let dir = state.get_dir(&dir_name)?;

        match dir {
            None => Err("no adr directory could be determined")?,
            Some(x) => self.create_adr(x, name),
        }
    }

    fn create_adr(&self, dir: &Directory, name: &str) -> Result<(), Box<dyn Error>> {
        let date = Utc::now().date();
        dir.create_adr(name, date, Status::Proposed, "", "", "")
    }
}
