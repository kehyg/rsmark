use crate::token::{node::Node};

pub mod block;
pub mod inline;


#[derive(Debug)]
pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
      return  Parser {  };
    }

    pub fn parser(&self, content: &str) -> Vec<Node> {
      // parser block
      let block_nodes = block::parser(content);

      return block_nodes;
    }
}