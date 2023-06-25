#![doc = include_str!("readme.md")]
use core::range::Range;

/// Root node of the TeX AST.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TexRoot {
    /// The span of the root node.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// The items contained in the root node.
    pub items: Vec<TexItem>,
}

/// A top-level item in a TeX document.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TexItem {
    /// A TeX command.
    Command(TexCommand),
    /// A TeX environment.
    Environment(TexEnvironment),
    /// A TeX group.
    Group(TexGroup),
    /// A TeX math environment.
    Math(TexMath),
    /// A superscript.
    Superscript(TexSuperscript),
    /// A subscript.
    Subscript(TexSubscript),
    /// Plain text.
    Text {
        /// The span of the text.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
        /// The content of the text.
        content: String,
    },
    /// A comment.
    Comment {
        /// The span of the comment.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
        /// The content of the comment.
        content: String,
    },
}

/// A TeX environment (e.g., \begin{matrix} ... \end{matrix}).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TexEnvironment {
    /// The span of the environment.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// The name of the environment.
    pub name: String,
    /// The arguments to the environment.
    pub arguments: Vec<TexArgument>,
    /// The content of the environment.
    pub content: TexRoot,
}

/// A TeX superscript (^).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TexSuperscript {
    /// The span of the superscript.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// The target of the superscript.
    pub target: Option<Box<TexItem>>,
    /// The content of the superscript.
    pub content: Box<TexRoot>,
}

/// A TeX subscript (_).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TexSubscript {
    /// The span of the subscript.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// The target of the subscript.
    pub target: Option<Box<TexItem>>,
    /// The content of the subscript.
    pub content: Box<TexRoot>,
}

/// A TeX math environment ($...$ or $$...$$).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TexMath {
    /// The span of the math environment.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// The content of the math environment.
    pub content: TexRoot,
    /// Whether this is a display math environment.
    pub is_display: bool,
}

/// A TeX command (e.g., \section, \textbf).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TexCommand {
    /// The span of the command.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// The name of the command.
    pub name: String,
    /// The arguments to the command.
    pub arguments: Vec<TexArgument>,
}

/// A TeX argument.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TexArgument {
    /// An optional argument ([...]).
    Optional(TexRoot),
    /// A mandatory argument ({...}).
    Required(TexRoot),
}

/// A TeX group (e.g., { ... }).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TexGroup {
    /// The span of the group.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// The content of the group.
    pub content: TexRoot,
}

impl TexRoot {
    /// Creates a new TeX root node.
    pub fn new(span: Range<usize>) -> Self {
        Self { span, items: Vec::new() }
    }
}
