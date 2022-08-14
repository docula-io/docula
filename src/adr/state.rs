use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use std::io::{Error,ErrorKind};
use super::Directory;

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub dirs: Vec<Directory>,
    #[serde(skip)]
    pub path: std::path::PathBuf,
}

impl State {
    pub fn new(path: &std::path::PathBuf) -> State {
        return State{
            dirs: Vec::new(),
            path: path.clone(),
        }
    }

    pub fn set_path(&mut self, path: &std::path::PathBuf) {
        self.path = path.clone();

        for dir in self.dirs.iter_mut() {
            dir.full_path = path.clone().join(dir.path.clone())
        }
    }

    pub fn get_dir(&self, name: &Option<String>) -> Result<Option<&Directory>, Error> {
        match name {
            Some(x) => self.get_named_dir(&x),
            None => self.get_current_dir(),
        }
    }

    fn get_named_dir(&self, name: &str) -> Result<Option<&Directory>, Error> {
        for dir in self.dirs.iter() {
            if dir.name == name {
                return Ok(Some(dir))
            }
        }

        Ok(None)
    }

    fn get_current_dir(&self) -> Result<Option<&Directory>, Error> {
        let cwd = std::env::current_dir()?;

        for dir in self.dirs.iter() {
            let fpath = self.path.join(&dir.path);

            if cwd.eq(&fpath) {
                return Ok(Some(dir))
            }
        }

        if self.dirs.len() == 1 {
            return Ok(self.dirs.first())
        }

        Ok(None)
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
