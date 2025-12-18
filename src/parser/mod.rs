use crate::token::Token;

pub mod block;
pub mod inline;

#[derive(Debug)]
pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        return Parser {};
    }

    pub fn parser<'a>(&self, content: &'a str) -> Vec<Token<'a>> {
        // parser block
        let mut tokens: Vec<Token<'a>> = block::Block::parser(content);

        tokens.iter_mut().for_each(|token| {
            let tokens = token.parser();
            token.children = tokens;
        });

        return tokens;
    }
}
