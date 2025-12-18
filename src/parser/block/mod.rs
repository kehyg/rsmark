use regex::Regex;
use serde::Serialize;
use std::sync::LazyLock;

use crate::{
    parser::inline::Inline,
    token::{
        Token,
        node::{Node, NodeType, Range},
    },
};

pub struct Block {}

impl Block {
    const BLOCK_SPLIT_MARK: &str =
        r"(?P<t1>[\n\s\S]*?)\s{2,}\n|(?P<t2>[\n\s\S]*?)\n\s{0,}\n|(?P<t3>[\n\s\S]*?)$";
    // const BLOCK_LINE_EMPTY: &str = r"^\s{1,}$";
    const RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(Self::BLOCK_SPLIT_MARK).unwrap());
    // const EMPTY_RE: LazyLock<Regex> =
    //     LazyLock::new(|| Regex::new(Block::BLOCK_LINE_EMPTY).unwrap());

    pub fn parser<'a>(content: &'a str) -> Vec<Token<'a>> {
        /* split line */
        let mut blocks: Vec<Token> = Self::RE
            .captures_iter(content)
            .map(|caps| {
                // println!("{:?}", caps);
                let mut token = Token {
                    text: content,
                    range: Range { start: 0, end: 0 },
                    iden: NodeType::Paragraph,
                    node: Box::new(Paragraph {}),
                    children: Vec::new(),
                };
                if let Some(_match) = caps.get(1) {
                    token.range.start = _match.start();
                    token.range.end = _match.end();
                }
                if let Some(_match) = caps.get(2) {
                    token.range.start = _match.start();
                    token.range.end = _match.end();
                }
                if let Some(_match) = caps.get(3) {
                    token.range.start = _match.start();
                    token.range.end = _match.end();
                }

                // if Self::EMPTY_RE.is_match(*x) {
                //     node.iden = NodeType::EmptyLine
                // }
                return token;
            })
            .collect();
        // println!("{:?}", blocks);
        /* check line type */
        Header::identify(&mut blocks);
        BlockQuote::identify(&mut blocks);
        Image::identify(&mut blocks);
        return blocks;
    }
}

#[derive(Debug, Serialize)]
pub struct Paragraph {}

impl<'a> Node<'a> for Paragraph {
    fn parser(&self, token: &Token<'a>) -> Vec<Token<'a>> {
        return Inline::parser(&token.text[token.range.start..token.range.end]);
    }
}

#[derive(Debug, Serialize)]
pub struct Header {
    pub range: Range,
}

impl Header {
    const RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^(?P<header>#{1,6})\s{1,}(?P<title>.*)").unwrap());

    fn level(header: &str) -> NodeType {
        let num = header.len();
        match num {
            1 => NodeType::H1,
            2 => NodeType::H2,
            3 => NodeType::H3,
            4 => NodeType::H4,
            5 => NodeType::H5,
            _ => NodeType::H6,
        }
    }
    fn identify(tokens: &mut Vec<Token>) -> bool {
        tokens.iter_mut().for_each(|token| {
            let range = &token.range;
            let text = &token.text[range.start..range.end];
            let caps = Self::RE.captures(text);
            if let Some(caps) = caps {
                token.iden = Self::level(&caps["header"]);
                if let Some(_match) = caps.get(2) {
                    token.node = Box::new(Header {
                        range: Range {
                            start: range.start + _match.start(),
                            end: range.start + _match.end(),
                        },
                    });
                }
            }
        });
        return true;
    }
}

impl<'a> Node<'a> for Header {
    fn parser(&self, token: &Token<'a>) -> Vec<Token<'a>> {
        // println!("{:?}", self);
        return Inline::parser(&token.text[token.range.start..token.range.end]);
    }
}

#[derive(Debug, Serialize)]
pub struct BlockQuote {
    range: Range,
}

impl BlockQuote {
    const RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^>\s{1}(?P<quote>[\s\S]*)").unwrap());
    fn identify(tokens: &mut Vec<Token>) -> bool {
        tokens.iter_mut().for_each(|token| {
            if token.iden != NodeType::Paragraph {
                return;
            }
            let range = &token.range;
            let text = &token.text[range.start..range.end];
            let caps = Self::RE.captures(text);
            if let Some(caps) = caps {
                token.iden = NodeType::BlockQuote;
                if let Some(_match) = caps.get(1) {
                    // println!("{:?}", _match.as_str());
                    token.node = Box::new(BlockQuote {
                        range: Range {
                            start: range.start + _match.start(),
                            end: range.start + _match.end(),
                        },
                    });
                }
            }
        });
        return true;
    }
}

const SPLIT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r">{0,}\s{1,}(.*?)\s{0,}\n>{0,}|>{0,}\s{1,}(.*?)$").unwrap());
impl<'a> Node<'a> for BlockQuote {
    fn parser(&self, token: &Token<'a>) -> Vec<Token<'a>> {
        let mut tokens: Vec<Token> = SPLIT_RE
            .captures_iter(&token.text[token.range.start..token.range.end])
            .map(|caps| {
                let mut _token = Token {
                    text: token.text,
                    range: Range { start: 0, end: 0 },
                    iden: NodeType::Paragraph,
                    node: Box::new(Paragraph {}),
                    children: Vec::new(),
                };
                if let Some(_match) = caps.get(1) {
                    _token.range.start = token.range.start + _match.start();
                    _token.range.end = token.range.start + _match.end();
                }
                if let Some(_match) = caps.get(2) {
                    _token.range.start = token.range.start + _match.start();
                    _token.range.end = token.range.start + _match.end();
                }
                return _token;
            })
            .collect();
        tokens.iter_mut().for_each(|token| {
            let tokens = token.parser();
            token.children = tokens;
        });
        return tokens;
    }
}

#[derive(Debug, Serialize)]
pub struct Image {
    description_range: Range,
    link_range: Range,
}

impl Image {
    const RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^!\[(.*?)\]\((.*?)\)").unwrap());
    fn identify(tokens: &mut Vec<Token>) -> bool {
        tokens.iter_mut().for_each(|token| {
            if token.iden != NodeType::Paragraph {
                return;
            }
            let range = &token.range;
            let text = &token.text[range.start..range.end];
            let caps = Self::RE.captures(text);
            if let Some(caps) = caps {
                token.iden = NodeType::Image;
                let mut node = Image {
                    description_range: Range { start: 0, end: 0 },
                    link_range: Range { start: 0, end: 0 },
                };
                match caps.get(1) {
                    Some(_match) => {
                        node.description_range.start = range.start + _match.start();
                        node.description_range.end = range.start + _match.end();
                    }
                    None => {}
                }
                match caps.get(2) {
                    Some(_match) => {
                        node.link_range.start = range.start + _match.start();
                        node.link_range.end = range.start + _match.end();
                    }
                    None => {}
                }
                token.node = Box::new(node);
            }
        });
        return true;
    }
}

impl<'a> Node<'a> for Image {
    fn parser(&self, _token: &Token<'a>) -> Vec<Token<'a>> {
        return Vec::new();
    }
}
