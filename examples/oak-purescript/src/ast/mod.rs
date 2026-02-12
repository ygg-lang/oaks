#![doc = include_str!("readme.md")]
/// PureScript AST root node.
#[derive(Debug, Clone)]
pub struct PurescriptRoot {
    pub elements: Vec<Element>,
}

/// PureScript syntax elements.
#[derive(Debug, Clone)]
pub enum Element {
    Module(String),
    Import(String),
    DataDecl(String),
    FunctionDecl(String),
    Identifier(String),
    Keyword(String),
    Operator(String),
    StringLiteral(String),
    NumberLiteral(String),
    CharLiteral(String),
    Comment(String),
    Whitespace(String),
    Newline,
}
