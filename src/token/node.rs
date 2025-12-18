use serde::Serialize;
use std::fmt::Debug;

use crate::token::Token;

#[derive(Debug, PartialEq, Serialize)]
pub enum NodeType {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Paragraph,
    BlockQuote,
    Image,
    Text,
    Quote,
    Bold,
    Link,
}

#[derive(Debug, Serialize)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

pub trait Node<'a>: Debug {
    fn parser(&self, token: &Token<'a>) -> Vec<Token<'a>>;
}
