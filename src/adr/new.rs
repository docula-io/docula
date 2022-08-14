use clap::Args;
use std::error::Error;
use chrono::Utc;
use inflector::Inflector;
use super::state::IndexType;

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
        let index = dir.next_index()?;

        let filename = format!("{}-{}.md", index, self.name.trim().replace(" ", "-"));

        let name = match dir.index {
            IndexType::Sequential => format!("{} {}", index, self.name),
            IndexType::Timestamp => format!("{}", self.name),
        };

        let date = format!("{}", Utc::now().format("%Y-%m-%d"));

        if !dir.full_path.exists() {
            std::fs::create_dir_all(&dir.full_path)?;
        }

        let template = format!(
            "# {}\n\n\
            Date: {}\n\n\
            ## Status\n\n\
            Proposed | Accepted\n\n\
            ## Context\n\n\
            Enter context here\n\n\
            ## Decision\n\n\
            Enter decision here\n\n\
            ## Consequences\n\n\
            Enter consequences here\n",
            name.to_title_case(), date,
        );

        let path = dir.full_path.join(filename);

        std::fs::write(path, template)?;

        Ok(())
    }
}
