#![doc = include_str!("readme.md")]
/// Root node of the Windows Command (CMD) syntax tree.
#[derive(Debug, Clone)]
pub struct CmdRoot {
    /// Elements in the syntax tree.
    pub elements: Vec<Element>,
}

/// Element in the Windows Command (CMD) syntax tree.
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
    /// Label.
    Label(String),
    /// Keyword.
    Keyword(String),
    /// Text.
    Text(String),
}
