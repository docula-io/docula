use serde::{Serialize, Deserialize};
use crate::adr;

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub adr: adr::state::State,
    #[serde(skip)]
    pub path: std::path::PathBuf,
}

impl State {
    pub fn load() -> Result<State, std::io::Error> {
        let existing = find_state_path()?;

        if existing.is_none() {
            return State::new()
        }

        let path = existing.unwrap();

        let contents = std::fs::read_to_string(&path)?;

        let mut state: State = serde_yaml::from_str(&contents).unwrap();

        state.path = path.parent().unwrap().to_path_buf();

        Ok(state)
    }

    pub fn new() -> Result<State, std::io::Error> {
        return Ok(State{
            adr: adr::state::State::new(),
            path: std::env::current_dir()?,
        })
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let encoded = serde_yaml::to_string(self).unwrap();
        let path = self.path.join(".docula");

        std::fs::write(path, encoded)
    }
}

fn find_state_path() -> Result<Option<std::path::PathBuf>, std::io::Error> {
    let path = std::env::current_dir()?;
    check_path(&path)
}

fn check_path(path: &std::path::Path) -> Result<Option<std::path::PathBuf>, std::io::Error> {
    let p = path.join(".docula");
    if p.exists() {
        return Ok(Some(p));
    }

    if p.is_dir() {
        return Err(std::io::Error::new(std::io::ErrorKind::Unsupported, ""))
    }

    match path.parent() {
        None => Ok(None),
        Some(x) => check_path(x),
    }
}
