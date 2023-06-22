use core::range::Range;
use serde::{Deserialize, Serialize};

/// Strongly-typed AST root node for Handlebars templates.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HandlebarsRoot {
    pub nodes: Vec<TemplateNode>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TemplateNode {
    Content(Content),
    Mustache(Mustache),
    Block(Block),
    Comment(Comment),
    Partial(Partial),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Content {
    pub text: String,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Mustache {
    pub expression: Expression,
    pub is_unescaped: bool,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Block {
    pub name: String,
    pub params: Vec<Expression>,
    pub body: Vec<TemplateNode>,
    pub inverse: Option<Vec<TemplateNode>>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Comment {
    pub text: String,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Partial {
    pub name: String,
    pub params: Vec<Expression>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    Path(String),
    Literal(Literal),
    SubExpression(Box<Expression>),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
}
