use serde::{Serialize, Deserialize};
use super::{Adr, IndexType, Status};
use chrono::{Date, Utc};
use inflector::Inflector;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Directory {
    pub path: std::path::PathBuf,
    pub name: String,
    pub index: IndexType,

    #[serde(skip)]
    pub full_path: std::path::PathBuf,
}

impl Directory {
    pub fn get_adrs(&self) -> Result<Vec<Adr>, Box<dyn std::error::Error>> {
        let mut res = Vec::new();

        for entry in self.full_path.read_dir()?.flatten() {
            if let Some(adr) = Adr::load(&entry.path())? {
                res.push(adr)
            }
        }

        Ok(res)
    }

    pub fn create_adr(
        &self, title: &str, date: Date<Utc>, status: Status, context: &str,
        decision: &str, consequences: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let index = self.next_index()?;

        let filename = format!(
            "{}-{}.md", index, title.trim().replace(' ', "-").to_lowercase(),
        );

        if !self.full_path.exists() {
            std::fs::create_dir_all(&self.full_path)?;
        }

        let template = format!(
            "# {}\n\n\
            Date: {}\n\n\
            ## Status\n\n\
            {} \n\n\
            ## Context\n\n\
            {}\n\n\
            ## Decision\n\n\
            {}\n\n\
            ## Consequences\n\n\
            {}\n",
            title.to_title_case(), 
            date.format("%Y-%m-%d"),
            status,
            context,
            decision,
            consequences
        );

        let path = self.full_path.join(filename);

        std::fs::write(path, template)?;

        Ok(())
    }

    pub fn next_index(&self) -> Result<String, Box<dyn std::error::Error>> {
        match &self.index {
            IndexType::Sequential => Ok(format!("{:05}", self.get_seq_index()?)),
            IndexType::Timestamp => Ok(format!("{}", Utc::now().format("%Y%m%d%H%M%S"))),
        }
    }

    fn get_seq_index(&self) -> Result<u32, Box<dyn std::error::Error>> {
        let mut max_index = 0;

        for entry in (self.full_path.read_dir()?).flatten() {
            if let Some(idx) = self.index_from_entry(entry) {
                if max_index < idx {
                    max_index = idx
                }
            }
        }

        Ok(max_index + 1)
    }

    fn index_from_entry(&self, entry: std::fs::DirEntry) -> Option<u32> {
        let adr = Adr::load(&entry.path()).ok()??;
        
        match adr.index.parse::<u32>() {
            Ok(x) => Some(x),
            Err(_) => None,
        }
    }
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
        std::fs::write(path, "# Something")?;

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
