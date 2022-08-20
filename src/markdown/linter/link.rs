use url::Url;

#[derive(Debug, PartialEq)]
pub struct FoundLink<'a> {
    start: usize,
    end: usize,
    link: LinkType,
    content: ContentType<'a>,
}

#[derive(Debug, PartialEq)]
pub enum ContentType<'a> {
    Image(&'a str),
    HyperLink(&'a str),
    Text,
}

#[derive(Debug, PartialEq)]
pub enum LinkType {
    File(std::path::PathBuf),
    Url(Url),
}

pub fn find_links(content: &str) -> Vec<FoundLink> {
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

    macro_rules! find_links_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $value;
                    assert_eq!(expected, find_links(input))
                }
            )*
        }
    }

    find_links_tests! {
        single_hyperlink: ("Hello, world [foo](https://bar.com)", vec![FoundLink{
            start: 13,
            end: 35,
            link: LinkType::Url(Url::parse("https://bar.com").unwrap()),
            content: ContentType::HyperLink("foo"),
        }]),
        single_imagelink: (
            "Hey, this image is great! ![Alt text](https://imgy.io/image.png) Cool!",
            vec![FoundLink{
                start: 26,
                end: 64,
                link: LinkType::Url(Url::parse("https://imgy.io/image.png").unwrap()),
                content: ContentType::Image("Alt text"),
            }]
        ),
    }
}

