use super::token;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref ATX_HEADER_REGEX: Regex = Regex::new(r"^\s*(#{1,6})\s*(.*?)\s*#*$").unwrap();
}

pub fn lex_analysis(input: &str) -> token::Document {
    let mut document = token::Document::new();
    let lines: Vec<&str> = input.lines().collect();

    for (num, line) in lines.iter().enumerate() {
        if let Some(x) = atx_header_from_line(line, num) {
            document.push(x);
            continue
        }
    }

    document
}

fn atx_header_from_line(line: &str, num: usize) -> Option<token::BlockToken> {
    let caps = ATX_HEADER_REGEX.captures(line)?;
    let depth = caps.get(1)?.as_str().len();
    let cap = caps.get(2)?;

    Some(token::BlockToken{
        line_start: num,
        token: token::Block::Heading {
            level: depth,
            content: get_content(cap.as_str(), num, cap.start()),
            style: token::HeadingStyle::Atx,
        }
    })
}

fn get_content(s: &str, line: usize, position: usize) -> token::Text {
    vec![
        token::InlineToken{
            line_start: line,
            position,
            token: token::Inline::Chunk(s.to_string()),
        }
    ]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_str() {
        let input = "";
        let expected: token::Document = vec![];
        let result = lex_analysis(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_atx_header_1_str() {
        let input = "# Header 1";
        let expected: token::Document = vec![
            token::BlockToken{
                line_start: 0,
                token: token::Block::Heading {
                    level: 1, 
                    style: token::HeadingStyle::Atx,
                    content: vec![
                        token::InlineToken {
                            line_start: 0,
                            position: 2,
                            token: token::Inline::Chunk("Header 1".to_owned()),
                        },
                    ],
                },
            },
        ];
        let result = lex_analysis(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_atx_header_1_str_wrapped() {
        let input = "# Header 1 #";
        let expected: token::Document = vec![
            token::BlockToken{
                line_start: 0,
                token: token::Block::Heading {
                    level: 1, 
                    style: token::HeadingStyle::Atx,
                    content: vec![
                        token::InlineToken {
                            line_start: 0,
                            position: 2,
                            token: token::Inline::Chunk("Header 1".to_owned()),
                        },
                    ],
                },
            },
        ];
        let result = lex_analysis(input);

        assert_eq!(expected, result);
    }
}
