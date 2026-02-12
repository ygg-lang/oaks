#![doc = include_str!("readme.md")]
use core::range::Range;

/// The root of a Tcl Abstract Syntax Tree.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TclRoot {
    /// The span of the entire Tcl script in the source file.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// The items (commands or comments) in the script.
    pub items: Vec<TclItem>,
}

/// A top-level item in a Tcl script.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TclItem {
    /// A Tcl command.
    Command(TclCommand),
    /// A Tcl comment.
    Comment(TclComment),
}

/// A Tcl command.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TclCommand {
    /// The span of the command in the source file.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// The words (arguments) of the command.
    pub words: Vec<TclWord>,
}

/// A Tcl word (an argument or command name).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TclWord {
    /// A simple string word.
    Simple(String),
    /// A variable substitution word.
    Variable(String),
    /// A script substitution word.
    Script(TclRoot),
    /// A braced string word.
    Braced(String),
}

/// A Tcl comment.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TclComment {
    /// The span of the comment in the source file.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// The text of the comment.
    pub text: String,
}

impl TclRoot {
    /// Creates a new Tcl root node with the given span.
    pub fn new(span: Range<usize>) -> Self {
        Self { span, items: Vec::new() }
    }
}
