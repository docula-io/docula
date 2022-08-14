use clap::Args;
use super::Directory;
use std::error::Error;

#[derive(Debug, Args)]
pub struct ListArgs {
    #[clap(short, long, value_parser, help = "The name of the ADR dir")]
    name: Option<String>,
}

impl ListArgs {
    pub fn handle(self) -> Result<(), Box<dyn Error>> {
        let state = crate::state::State::load()?.adr;

        if state.dirs.len() == 0 {
            Err("please set up an adr dir using the `init` command")?;
        }

        let dir = state.get_dir(&self.name)?;

        let output = match dir {
            None => Err("no adr directory could be determined")?,
            Some(x) => self.list_output(x)?,
        };

        println!("{}", output);
        
        Ok(())
    }

    fn list_output(&self, dir: &Directory) -> Result<String, Box<dyn Error>> {
        let adrs = dir.get_adrs()?;

        let mut res = String::new();

        for adr in adrs.iter() {
            res.push_str(format!("{} - {}\n", adr.index, adr.title).as_ref());
        }

        Ok(res)
    }
}
