#![doc = include_str!("readme.md")]
/// PureScript AST root node.
#[derive(Debug, Clone)]
pub struct PurescriptRoot {
    /// The list of syntax elements in the PureScript source.
    pub elements: Vec<Element>,
}

/// PureScript syntax elements.
#[derive(Debug, Clone)]
pub enum Element {
    /// A module declaration.
    Module(String),
    /// An import statement.
    Import(String),
    /// A data type declaration.
    DataDecl(String),
    /// A function declaration.
    FunctionDecl(String),
    /// An identifier.
    Identifier(String),
    /// A keyword.
    Keyword(String),
    /// An operator.
    Operator(String),
    /// A string literal.
    StringLiteral(String),
    /// A number literal.
    NumberLiteral(String),
    /// A character literal.
    CharLiteral(String),
    /// A comment.
    Comment(String),
    /// Whitespace.
    Whitespace(String),
    /// A newline character.
    Newline,
}
