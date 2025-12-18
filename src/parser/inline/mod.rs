use std::sync::LazyLock;

use regex::Regex;
use serde::Serialize;

use crate::token::{
    Token,
    node::{Node, NodeType, Range},
};

pub struct Inline {}

impl Inline {
    pub fn handle_prev<'a>(
        prev_start: usize,
        current_start: usize,
        content: &'a str,
    ) -> Option<Token<'a>> {
        if prev_start < current_start {
            // have text
            let _token = Token {
                text: content,
                iden: NodeType::Text,
                range: Range {
                    start: prev_start,
                    end: current_start,
                },
                node: Box::new(Text {
                    range: Range {
                        start: prev_start,
                        end: current_start,
                    },
                }),
                children: Vec::new(),
            };
            return Some(_token);
        }
        return None;
    }

    pub fn next_index(index: &Vec<usize>, mut k: usize, v: usize) -> usize {
        while k < index.len() {
            if index[k] == v {
                return k;
            }
            k = k + 1;
        }
        return k;
    }

    pub fn parser<'a>(content: &'a str) -> Vec<Token<'a>> {
        let chars_index: Vec<usize> = content.char_indices().map(|(i, _)| i).collect();
        let end_index = chars_index.len();
        let mut next_index: usize = 0;
        let mut prev_end: usize = 0;
        let mut tokens: Vec<Token> = Vec::new();
        while next_index < end_index {
            // [2,3,4,6,8,9,10]
            //  ↑
            // next_index = 0
            let start_index = chars_index[next_index];
            let text = &content[start_index..content.len()];

            if let Some(mut token) = Quote::parser(text) {
                token.range.start += start_index;
                token.range.end += start_index;
                token.text = content;
                if let Some(token) = Inline::handle_prev(prev_end, start_index, content) {
                    tokens.append(&mut vec![token]);
                }
                prev_end = token.range.end;
                tokens.append(&mut vec![token]);
                next_index = Inline::next_index(&chars_index, next_index, prev_end);
                continue;
            }
            if let Some(mut token) = Bold::parser(text) {
                token.range.start += start_index;
                token.range.end += start_index;
                token.text = content;
                if let Some(token) = Inline::handle_prev(prev_end, start_index, content) {
                    tokens.append(&mut vec![token]);
                }
                prev_end = token.range.end;
                tokens.append(&mut vec![token]);
                next_index = Inline::next_index(&chars_index, next_index, prev_end);
                continue;
            }
            if let Some(mut token) = Link::parser(text) {
                token.range.start += start_index;
                token.range.end += start_index;
                token.text = content;
                if let Some(token) = Inline::handle_prev(prev_end, start_index, content) {
                    tokens.append(&mut vec![token]);
                }
                prev_end = token.range.end;
                tokens.append(&mut vec![token]);
                next_index = Inline::next_index(&chars_index, next_index, prev_end);
                continue;
            }
            next_index = next_index + 1;
        }
        if let Some(token) = Inline::handle_prev(prev_end, chars_index[end_index - 1], content) {
            tokens.append(&mut vec![token]);
        }
        return tokens;
    }
}

#[derive(Debug, Serialize)]
pub struct Text {
    pub range: Range,
}

impl<'a> Node<'a> for Text {
    fn parser(&self, _token: &Token<'a>) -> Vec<Token<'a>> {
        todo!()
    }
}

#[derive(Debug, Serialize)]
pub struct Quote {
    pub range: Range,
}

impl Quote {
    const RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^`(.*?)`").unwrap());
    fn parser<'a>(content: &'a str) -> Option<Token<'a>> {
        let caps = Self::RE.captures(content);
        if let Some(caps) = caps {
            let _match = caps.get(0).unwrap();
            let _match2 = caps.get(1).unwrap();
            let node = Token {
                text: _match.as_str(),
                iden: NodeType::Quote,
                range: Range {
                    start: _match.start(),
                    end: _match.end(),
                },
                node: Box::new(Quote {
                    range: Range {
                        start: _match2.start(),
                        end: _match2.end(),
                    },
                }),
                children: Vec::new(),
            };
            return Some(node);
        }

        return None;
    }
}

impl<'a> Node<'a> for Quote {
    fn parser(&self, _token: &Token<'a>) -> Vec<Token<'a>> {
        todo!()
    }
}

#[derive(Debug, Serialize)]
pub struct Bold {
    pub range: Range,
}

impl Bold {
    const RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\*(.*)\*").unwrap());
    fn parser<'a>(content: &'a str) -> Option<Token<'a>> {
        let caps = Self::RE.captures(content);
        if let Some(caps) = caps {
            let _match = caps.get(0).unwrap();
            let _match2 = caps.get(1).unwrap();
            let node = Token {
                iden: NodeType::Bold,
                range: Range {
                    start: _match.start(),
                    end: _match.end(),
                },
                text: _match.as_str(),
                node: Box::new(Bold {
                    range: Range {
                        start: _match2.start(),
                        end: _match2.end(),
                    },
                }),
                children: Vec::new(),
            };
            return Some(node);
        }
        return None;
    }
}

impl<'a> Node<'a> for Bold {
    fn parser(&self, _token: &Token<'a>) -> Vec<Token<'a>> {
        todo!()
    }
}

#[derive(Debug, Serialize)]
pub struct Link {
    pub description_range: Range,
    pub link_range: Range,
}

impl Link {
    const RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^!{0,1}\[(.*?)\]\((.*?)\)").unwrap());
    fn parser<'a>(content: &'a str) -> Option<Token<'a>> {
        let caps = Self::RE.captures(content);
        if let Some(caps) = caps {
            let _match = caps.get(0).unwrap();
            let _match1 = caps.get(1).unwrap();
            let _match2 = caps.get(2).unwrap();
            let node = Token {
                iden: NodeType::Link,
                range: Range {
                    start: _match.start(),
                    end: _match.end(),
                },
                text: _match.as_str(),
                node: Box::new(Link {
                    description_range: Range {
                        start: _match1.start(),
                        end: _match1.end(),
                    },
                    link_range: Range {
                        start: _match2.start(),
                        end: _match2.end(),
                    },
                }),
                children: vec![],
            };
            return Some(node);
        }
        return None;
    }
}

impl<'a> Node<'a> for Link {
    fn parser(&self, _token: &Token<'a>) -> Vec<Token<'a>> {
        todo!()
    }
}
