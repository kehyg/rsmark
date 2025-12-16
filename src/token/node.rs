
#[derive(Debug)]
pub enum NodeType {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Paragraph,
    Text,
    Quote,
    Image,
    Bold,
    Link 
}

#[derive(Debug)]
pub struct Range {
  pub line: i32,
  pub start: i32,
  pub len: i32,
}

#[derive(Debug)]
pub struct Node {
  // pub text: String,
  pub range: Range,
  pub iden: NodeType,
}