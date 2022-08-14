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
        let state = crate::state::State::load()?.adr;

        if state.dirs.len() == 0 {
            Err("please set up an adr dir using the `init` command")?;
        }

        let dir = state.get_dir(&self.dir_name)?;

        match dir {
            None => Err("no adr directory could be determined")?,
            Some(x) => self.create_adr(x),
        }
    }

    fn create_adr(&self, dir: &super::state::Directory) -> Result<(), Box<dyn Error>> {
        let template = "# ";
        Ok(())
    }
}
