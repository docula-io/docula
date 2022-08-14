use clap::ValueEnum;
use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use std::io::{Error,ErrorKind};
use chrono::prelude::*;

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

            println!("{:?}", fpath);
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Directory {
    pub path: std::path::PathBuf,
    pub name: String,
    pub index: IndexType,

    #[serde(skip)]
    pub full_path: std::path::PathBuf,
}


impl Directory {
    pub fn next_index(&self) -> Result<String, Box<dyn std::error::Error>> {
        match &self.index {
            IndexType::Sequential => Ok(format!("{:05}", self.get_seq_index()?)),
            IndexType::Timestamp => Ok(format!("{}", Utc::now().format("%Y%m%d%H%M%S"))),
        }
    }

    fn get_seq_index(&self) -> Result<u32, Box<dyn std::error::Error>> {
        let mut max_index = 0;

        for x in self.full_path.read_dir()? {
            if let Ok(entry) = x {
                if let Some(idx) = self.index_from_entry(entry) {
                    if max_index < idx {
                        max_index = idx
                    }
                }
            }
        }

        return Ok(max_index + 1)
    }

    fn index_from_entry(&self, entry: std::fs::DirEntry) -> Option<u32> {
        let re = regex::Regex::new(r"^(\d{5})-.*\.md$").unwrap();

        let path = entry.path();
        let name = path.file_name()?.to_str()?;

        if !re.is_match(name) {
            return None;
        }

        let caps = re.captures(name)?;
        let res = caps.get(1).map_or("", |m| m.as_str());
        
        match u32::from_str_radix(res, 10) {
            Ok(x) => Some(x),
            Err(_) => None,
        }
    }
}

#[derive(Serialize, Deserialize, ValueEnum, Debug, Clone)]
pub enum IndexType {
    Timestamp,
    Sequential,
}

#[cfg(test)]
mod test {
    use super::{Directory, IndexType};
    use chrono::Utc;
    use std::error::Error;

    #[test]
    fn test_directory_index_timestamp() -> Result<(), Box<dyn Error>> {
        let tmp = tempdir::TempDir::new("dir_test")?;

        let dir = Directory{
            path: tmp.path().to_path_buf(),
            name: "foo".to_owned(),
            index: IndexType::Timestamp,
            full_path: tmp.path().to_path_buf(),
        };

        let now = Utc::now();
        let idx = dir.next_index()?;

        let expected = format!("{}", now.format("%Y%m%d%H%M%S"));
        assert_eq!(idx, expected);

        Ok(())
    }

    #[test]
    fn test_directory_index_sequential() -> Result<(), Box<dyn Error>> {
        let tmp = tempdir::TempDir::new("dir_test")?;

        let dir = Directory{
            path: tmp.path().to_path_buf(),
            name: "foo".to_owned(),
            index: IndexType::Sequential,
            full_path: tmp.path().to_path_buf(),
        };

        let index = dir.next_index()?;
        assert_eq!("00001", index);

        // Add in a file with an index
        let path = tmp.path().join("00001-something.md");
        std::fs::write(path, "# 00001 Something")?;

        let index = dir.next_index()?;
        assert_eq!("00002", index);

        // Add in a non matching file
        let path = tmp.path().join("README.md");
        std::fs::write(path, "# ADR Directory")?;

        let index = dir.next_index()?;
        assert_eq!("00002", index);

        // Skip an index
        let path = tmp.path().join("00003-skipped-index.md");
        std::fs::write(path, "# ADR Directory")?;

        let index = dir.next_index()?;
        assert_eq!("00004", index);

        Ok(())
    }
}
