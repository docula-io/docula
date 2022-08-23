use super::token;
use lazy_static::lazy_static;
use regex::Regex;
use std::iter::{Enumerate, Iter};

lazy_static! {
    static ref ATX_HEADING_REGEX: Regex = Regex::new(r"^\s*(#{1,6})\s*(.*?)\s*#*$").unwrap();
    static ref SETEX_HEADING_1_REGEX: Regex = Regex::new(r"^\s*={2,}\s*$").unwrap();
    static ref SETEX_HEADING_2_REGEX: Regex = Regex::new(r"^\s*-{2,}\s*$").unwrap();
    static ref CODE_BLOCK_REGEX: Regex = Regex::new(r"^\s*```\s*$").unwrap();
}

pub fn lex_analysis(input: &str) -> token::Document {
    let mut document = token::Document::new();
    let lines: Vec<&str> = input.lines().collect();
    let mut skip = false;

    let iter = lines.iter().enumerate();

    for (num, line) in iter {
        if skip {
            skip = false;
            continue
        }

        if let Some(x) = atx_header_from_line(line, num) {
            document.push(x);
            continue
        }

        if let Some(x) = code_block(line, num, iter) {
            document.push(x);
            continue
        }

        if let Some(next) = lines.get(num + 1) {
            if let Some(x) = setex_header_from_line(line, next, num) {
                document.push(x);
                skip = true;
                continue;
            }
        }
    }

    document
}

fn atx_header_from_line(line: &str, num: usize) -> Option<token::BlockToken> {
    let caps = ATX_HEADING_REGEX.captures(line)?;
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

fn setex_header_from_line(line: &str, next: &str, num: usize) -> Option<token::BlockToken> {
    if SETEX_HEADING_1_REGEX.is_match(next) {
        Some(token::BlockToken{
            line_start: num,
            token: token::Block::Heading {
                level: 1,
                content: get_content(line, num, 0),
                style: token::HeadingStyle::Setex,
            }
        })
    } else if SETEX_HEADING_2_REGEX.is_match(next) {
        Some(token::BlockToken{
            line_start: num,
            token: token::Block::Heading {
                level: 2,
                content: get_content(line, num, 0),
                style: token::HeadingStyle::Setex,
            }
        })
    } else {
        None
    }
}

fn code_block(line: &str, num: usize, iter: Enumerate<Iter<&str>>) -> Option<token::BlockToken> {
    None
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

    #[test]
    fn test_settx_header_1_str() {
        let input = "Header 1\n=====";
        let expected: token::Document = vec![
            token::BlockToken{
                line_start: 0,
                token: token::Block::Heading {
                    level: 1,
                    style: token::HeadingStyle::Setex,
                    content: vec![
                        token::InlineToken {
                            line_start: 0,
                            position: 0,
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
    fn test_settx_header_2_str() {
        let input = "Header 2\n----";
        let expected: token::Document = vec![
            token::BlockToken{
                line_start: 0,
                token: token::Block::Heading {
                    level: 2,
                    style: token::HeadingStyle::Setex,
                    content: vec![
                        token::InlineToken {
                            line_start: 0,
                            position: 0,
                            token: token::Inline::Chunk("Header 2".to_owned()),
                        },
                    ],
                },
            },
        ];
        let result = lex_analysis(input);

        assert_eq!(expected, result);
    }
}
