use clap::Args;
use std::error::Error;

#[derive(Debug, Args)]
pub struct InitCmd {
    #[clap(help = "The directory where the adrs will live")]
    dir: std::path::PathBuf,
    #[clap(short, long, value_parser, help = "The name of this adr directory")]
    name: String,
    #[clap(short, long, value_enum, default_value="timestamp")]
    index_type: super::IndexType
}

impl InitCmd {
    pub fn handle<T: InitHandler>(self, h: T) -> Result<(), Box<dyn Error>> {
        h.handle_cmd(self)
    }
}

pub trait InitHandler {
    fn handle_cmd(&self, cmd: InitCmd) -> Result<(), Box<dyn Error>>;
}

pub struct Handler {
}

impl InitHandler for Handler {
    fn handle_cmd(&self, cmd: InitCmd) -> Result<(), Box<dyn Error>> {
        let mut state = crate::state::State::load()?;

        let cwd = std::env::current_dir()?;

        let adr_path = cwd.join(&cmd.dir);

        if !adr_path.exists() {
            std::fs::create_dir_all(&adr_path)?;
        }

        let canon_path = adr_path.canonicalize()?;

        if !path_is_parent(&state.path, &canon_path) {
            println!("{:?} {:?}", state.path, canon_path);
            return Err("path is not correct")?;
        }

        let parent_count = state.path.components().count();

        let relative_path: std::path::PathBuf = canon_path.components().skip(parent_count).collect();

        let dir = super::state::Directory{
            path: relative_path,
            name: cmd.name,
            index: cmd.index_type,
            full_path: canon_path,
        };

        state.adr.validate_dir(&dir)?;

        state.adr.dirs.push(dir);

        state.save()?;

        Ok(())
    }
}

fn path_is_parent(parent: &std::path::PathBuf, path: &std::path::PathBuf) -> bool {
    if path == parent {
        return true
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
