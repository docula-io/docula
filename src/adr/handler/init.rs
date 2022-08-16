use crate::adr::{Directory, IndexType, Status};
use chrono::Utc;

pub struct Handler {}

impl Handler {
    pub fn handle(
        &self,
        dir: &std::path::Path,
        name: String,
        index_type: IndexType,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut state = crate::state::State::load()?;

        let cwd = std::env::current_dir()?;

        let adr_path = cwd.join(dir);

        if !adr_path.exists() {
            std::fs::create_dir_all(&adr_path)?;
        }

        let canon_path = adr_path.canonicalize()?;

        if !path_is_parent(&state.path, &canon_path) {
            println!("{:?} {:?}", state.path, canon_path);
            return Err("path is not correct")?;
        }

        let parent_count = state.path.components().count();

        let relative_path: std::path::PathBuf =
            canon_path.components().skip(parent_count).collect();

        let dir = Directory {
            path: relative_path,
            name,
            index: index_type,
            full_path: canon_path,
        };

        state.adr.validate_dir(&dir)?;

        state.adr.dirs.push(dir.clone());

        state.save()?;

        let date = Utc::now().date();

        dir.create_adr(
            "Record architecture decisions",
            date,
            Status::Accepted,
            "We need to record the architectural decisions made on this project.",
            "We will use Architecture Decision Records, managed by \
            [Docula](https://github.com/docula-io/docula),\n\
            as described by Michael Nygard in this article: \
            http://thinkrelevance.com/blog/2011/11/15/documenting-architecture-decisions",
            "See Michael Nygard's article, linked above.",
        )
    }
}

fn path_is_parent(parent: &std::path::PathBuf, path: &std::path::PathBuf) -> bool {
    if path == parent {
        return true;
    }

    match path.parent() {
        None => false,
        Some(x) => path_is_parent(parent, &x.to_path_buf()),
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_parent_path() {
        let parent = std::path::Path::new("/tmp").to_path_buf();

        let path = std::path::Path::new("/tmp/foo/bar").to_path_buf();
        assert!(super::path_is_parent(&parent, &path));

        let path = std::path::Path::new("/foo").to_path_buf();
        assert!(!super::path_is_parent(&parent, &path));
    }
}
