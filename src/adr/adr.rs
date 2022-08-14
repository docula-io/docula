use inflector::Inflector;
use chrono::{Date, NaiveDate ,Utc};
use std::fmt;

pub struct Adr {
    pub index: String,
    pub title: String,
    pub content: String,
    pub date: Option<chrono::Date<chrono::Utc>>,
}

pub enum Status {
    Accepted,
    Proposed
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::Accepted => write!(f, "Accepted"),
            Status::Proposed => write!(f, "Proposed"),
        }
    }
}

impl Adr {
    pub fn load(path: &std::path::PathBuf) -> Result<Option<Adr>, std::io::Error> {
        let re: regex::Regex = regex::Regex::new(r"^(\d{5})-(.*)\.md$").unwrap();

        let fname = match filename_from_path(path) {
            None => return Ok(None),
            Some(x) => x,
        };

        let caps = match re.captures(fname) {
            None => return Ok(None),
            Some(x) => x,
        };

        let index = match caps.get(1).map_or(None, |m| Some(m.as_str())) {
            None => return Ok(None),
            Some(x) => x,
        };

        let content = std::fs::read_to_string(path)?;

        let filename = match caps.get(2).map_or(None, |m| Some(m.as_str())) {
            None => return Ok(None),
            Some(x) => x,
        };

        let title = match title_from_content(&content).or(title_from_filename(filename)) {
            None => return Ok(None),
            Some(x) => x,
        };

        let date = date_from_content(&content);

        Ok(Some(Adr{
            index: index.to_owned(),
            content,
            title,
            date,
        }))
    }
}

fn filename_from_path(path: &std::path::PathBuf) -> Option<&str> {
    Some(path.file_name()?.to_str()?)
}

fn title_from_content(content: &str) -> Option<String> {
    let re: regex::Regex = regex::Regex::new(r"^#{1}\s{1,}(.*)[^\r\n]*").unwrap();

    let caps = re.captures(content)?;

    caps.get(1).map_or(None, |m| Some(m.as_str().trim().to_owned()))
}

fn title_from_filename(filename: &str) -> Option<String> {
    let rep = filename.replace("-", " ");
    let title = rep.trim();

    match title {
        "" => None,
        x => Some(x.to_title_case()),
    }
}

fn date_from_content(content: &str) -> Option<Date<Utc>> {
    let re: regex::Regex = regex::Regex::new(r"Date: (\d{4}-\d{2}-\d{2})").unwrap();

    let caps = re.captures(content)?;

    let date_str = caps.get(1).map_or(None, |m| Some(m.as_str().trim().to_owned()))?;

    let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").ok()?;

    Some(Date::<Utc>::from_utc(date, Utc))
}
