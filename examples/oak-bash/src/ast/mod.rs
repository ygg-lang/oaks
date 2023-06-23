#![doc = include_str!("readme.md")]
/// Root node of the Bash syntax tree.
#[derive(Debug, Clone)]
pub struct BashRoot {
    /// Elements in the syntax tree.
    pub elements: Vec<Element>,
}

/// Element in the Bash syntax tree.
#[derive(Debug, Clone)]
pub enum Element {
    /// Command.
    Command(String),
    /// Variable.
    Variable(String),
    /// String.
    String(String),
    /// Comment.
    Comment(String),
    /// Operator.
    Operator(String),
    /// Keyword.
    Keyword(String),
    /// Text.
    Text(String),
    /// Whitespace.
    Whitespace(String),
    /// Newline.
    Newline,
}
