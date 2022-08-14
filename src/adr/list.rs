use clap::Args;
use super::{Directory, Status};
use std::error::Error;
use tabled::{Tabled, Table};
use ansi_term::Colour;

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

        let mut rows = Vec::new();

        for adr in adrs.iter() {
            let status = match adr.status {
                Some(Status::Proposed) => Colour::Yellow.paint("Proposed"),
                Some(Status::Accepted) => Colour::Green.paint("Accepted"),
                None => Colour::Red.paint("Unknown"),
            };

            rows.push(AdrRow{
                index: &adr.index,
                title: &adr.title,
                date: adr.date.map_or(String::new(), |x| format!("{}", x.format("%Y-%m-%d"))),
                status: status.to_string(),
            });
        }

        rows.reverse();

        Ok(Table::new(rows).with(tabled::Style::modern()).to_string())
    }
}

#[derive(Tabled)]
struct AdrRow<'a> {
    index: &'a str,
    title: &'a str,
    date: String,
    status: String,
}
