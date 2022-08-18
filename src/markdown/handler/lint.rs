use url::Url;

pub struct Handler {}

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
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
struct FoundLink<'a> {
    start: usize,
    end: usize,
    link: LinkType,
    content: ContentType<'a>,
}

#[derive(Debug, PartialEq)]
enum ContentType<'a> {
    Image(&'a str),
    HyperLink(&'a str),
    Text,
}

#[derive(Debug, PartialEq)]
enum LinkType {
    File(std::path::PathBuf),
    Url(Url),
}

fn find_links(content: &str) -> Vec<FoundLink> {
    let re = regex::Regex::new(r"!?\[\s*(.*)\s*\]\(\s*(.*)\s*\)").unwrap();
    let mut res = Vec::new();

    for m in re.find_iter(content) {
        let text = m.as_str();

        let caps = re.captures(text);

        if caps.is_none() {
            continue;
        }

        let start = m.start();
        let end = m.end();

        let caps = caps.unwrap();
        let inner = caps.get(1).unwrap();
        let url = Url::parse(caps.get(2).unwrap().as_str()).unwrap();

        let content = match text.get(0..1).unwrap() {
            "!" => ContentType::Image(inner.as_str()),
            _ => ContentType::HyperLink(inner.as_str()),
        };

        res.push(FoundLink{
            start,
            end,
            link: LinkType::Url(url),
            content,
        })
    }

    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_finding_links() {
        let text = "Hello, world [foo](https://bar.com)";

        let res = find_links(text);

        let expected = FoundLink{
            start: 13,
            end: 35,
            link: LinkType::Url(Url::parse("https://bar.com").unwrap()),
            content: ContentType::HyperLink("foo"),
        };

        assert!(res.get(0).is_some());

        let res = res.get(0).unwrap();
        assert_eq!(res, &expected);
    }
}
