use std::fmt::Debug;

use serde::{Serialize, ser::SerializeStruct};

use crate::token::node::{Node, NodeType, Range};

pub mod node;

#[derive()]
pub struct Token<'a> {
    pub text: &'a str,
    pub range: Range,
    pub iden: NodeType,
    pub node: Box<dyn Node<'a>>,
    pub children: Vec<Token<'a>>,
}

impl<'a> Token<'a> {
    pub fn parser(&self) -> Vec<Token<'a>> {
        return self.node.parser(&self);
    }
}

impl<'a> Debug for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Token")
            .field("range", &self.range)
            .field("iden", &self.iden)
            .field("children", &self.children)
            .finish()
    }
}

impl<'a> Serialize for Token<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut reg = serializer.serialize_struct("token", 3)?;
        reg.serialize_field("range", &self.range)?;
        reg.serialize_field("iden", &self.iden)?;
        reg.serialize_field("children", &self.children)?;
        reg.end()
    }
}
