use clap::ValueEnum;
use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use std::io::{Error,ErrorKind};

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub dirs: Vec<Directory>
}

impl State {
    pub fn new() -> State {
        return State{
            dirs: Vec::new(),
        }
    }

    pub fn validate_dir(&self, dir: &Directory) -> Result<(), Error> {
        let paths: HashSet<std::path::PathBuf> = self.dirs.iter().map(|x| x.path.clone()).collect();
        let names: HashSet<&str> = self.dirs.iter().map(|x| x.name.as_ref()).collect();

        if paths.contains(&dir.path) {
            return Err(Error::new(ErrorKind::AlreadyExists, "path already exists"));
        }

        if names.contains(&dir.name.as_ref()) {
            return Err(Error::new(ErrorKind::AlreadyExists, "name already exists"));
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Directory {
    pub path: std::path::PathBuf,
    pub name: String,
    pub index: IndexType,
}

#[derive(Serialize, Deserialize, ValueEnum, Debug, Clone)]
pub enum IndexType {
    Timestamp,
    Sequential,
}
