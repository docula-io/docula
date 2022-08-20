use std::fmt::Display;
use rayon::prelude::*;

pub trait Linter: Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn lint(&self, content: &str) -> Vec<Issue>;
}

#[derive(Debug,PartialEq)]
pub struct Issue {
    pub line_start: usize,
    pub line_end: usize,
    pub col_start: usize,
    pub col_end: usize,
    pub content: String,
    pub msg: String
}

impl Display for Issue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

pub struct Handler {
    linters: Vec<Box<dyn Linter>>
}

impl Handler {
    pub fn handle(self, path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
        let path = std::env::current_dir()?.join(path).canonicalize()?;

        if path.is_dir() {
            return self.scan_dir();
        }

        match path.extension().and_then(|x| x.to_str()) {
            Some("md") => self.lint_file(&path),
            _ => Err("invalid file")?,
        }
    }

    fn scan_dir(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn lint_file(&self, path: &std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let issues: Vec<Issue> = self.linters.par_iter().flat_map(|x| x.lint(&content)).collect();

        for issue in issues {
           println!("{}", issue)
        }

        Ok(())
    }
}
