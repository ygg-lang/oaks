#![doc = include_str!("readme.md")]
use core::range::Range;

/// The root node of a Tailwind document.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TailwindRoot {
    /// The span of the entire root.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// The children of the root.
    pub children: Vec<TailwindNode>,
}

/// A node in the Tailwind AST.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TailwindNode {
    /// A CSS directive (e.g., @tailwind base).
    Directive(TailwindDirective),
    /// A utility class (e.g., bg-red-500).
    Class(TailwindClass),
    /// A comment.
    Comment(TailwindComment),
}

/// A Tailwind class (e.g., hover:bg-red-500, !p-4).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TailwindClass {
    /// The span of the entire class.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// Whether the class has an important flag (!).
    pub is_important: bool,
    /// The modifiers applied to the class.
    pub modifiers: Vec<TailwindModifier>,
    /// The kind of class (utility or arbitrary value).
    pub kind: TailwindClassKind,
}

/// The kind of Tailwind class.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TailwindClassKind {
    /// A utility class (e.g., `bg-red-500`).
    Utility(TailwindUtility),
    /// An arbitrary value (e.g., `[100px]`).
    ArbitraryValue(TailwindArbitraryValue),
}

/// A Tailwind modifier (e.g., hover:).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TailwindModifier {
    /// The span of the modifier.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// The name of the modifier.
    pub name: String,
}

/// A Tailwind utility (e.g., bg-red-500).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TailwindUtility {
    /// The span of the utility.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// The name of the utility.
    pub name: String,
}

/// A Tailwind arbitrary value (e.g., [100px]).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TailwindArbitraryValue {
    /// The span of the arbitrary value.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// The value inside the brackets.
    pub value: String,
}

/// A Tailwind directive (e.g., @tailwind base).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TailwindDirective {
    /// The span of the directive.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// The name of the directive (e.g., "tailwind").
    pub name: String,
    /// The optional body of the directive (e.g., "base").
    pub body: Option<String>,
}

/// A Tailwind comment.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TailwindComment {
    /// The span of the comment.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// The content of the comment.
    pub content: String,
}

impl TailwindRoot {
    /// Creates a new Tailwind root node with the given span.
    pub fn new(span: Range<usize>, children: Vec<TailwindNode>) -> Self {
        Self { span, children }
    }
}
