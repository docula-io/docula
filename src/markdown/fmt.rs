use clap::Args;

#[derive(Debug, Args)]
pub struct FmtCmd {
    path: std::path::PathBuf,
    #[clap(short, long, help="Recursively search and fmt markdown")]
    recursive: bool,
    #[clap(long, help="Print the output without making it")]
    dry_run: bool,
}

impl FmtCmd {
    pub fn handle(self) -> Result<(), Box<dyn std::error::Error>> {
        let path = std::env::current_dir()?.join(&self.path).canonicalize()?;

        if path.is_dir() {
            return self.scan_dir();
        }

        match path.extension().and_then(|x| x.to_str()) {
            Some("md") => self.fmt_file(&path),
            _ => Err("invalid file")?,
        }
    }

    pub fn scan_dir(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    pub fn fmt_file(&self, path: &std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let contents = std::fs::read_to_string(&path)?;

        let contents = fix_line_length(&contents);

        println!("{}", contents);
        Ok(())
    }
}

fn fix_line_length(s: &str) -> String {
    let mut res = String::new();
    let mut in_code_block = false;

    for line in s.lines() {
        if line.starts_with("```") {
            in_code_block = !in_code_block;
        }

        if !in_code_block {
            res.push_str(&split_line(line, 80).join("\n"));
        } else {
            res.push_str(line);
        }

        res.push('\n');
    }

    res
}

fn split_line(line: &str, width: usize) -> Vec<&str> {
    if line.trim().len() <= width {
        return vec![line.trim()];
    }

    match find_closest_space(line, width) {
        None => vec![line],
        Some(x) => {
            let parts = line.split_at(x + 1);

            let mut res = Vec::new();
            res.push(parts.0.trim());
            res.append(&mut split_line(parts.1, width));
            res
        }
    }
}

fn find_closest_space(line: &str, width: usize) -> Option<usize> {
    let mut best: Option<usize> = None;
    let mut best_offset: Option<usize> = None;
    let mut offset: usize = 0;
    let mut search = line;

    while let Some(x) = search.find(' ') {
        let delta = (((width + 1) as isize) - (x + offset) as isize).unsigned_abs();

        best = match best {
            None => {
                best_offset = Some(x);
                Some(delta)
            },
            Some(x) => {
                if x > delta {
                    best_offset = Some(offset - 1);
                    Some(delta)
                } else {
                    Some(x)
                }
            }
        };

        offset = x + offset + 1;
        search = search.split_at(x + 1).1;
    }

    best_offset
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_closest_space() {
        let line = "foo bar boo baz who am i to blame for thislonglong sentence";

        let res = find_closest_space(line, 45);

        assert_eq!(res, Some(37));
    }

    #[test]
    fn test_find_closest_space_longer_line() {
        let line = "supercalifragilisticexpialidocious is a real word";

        let res = find_closest_space(line, 20);

        assert_eq!(res, Some(34));
    }
}
