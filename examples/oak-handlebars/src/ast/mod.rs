#![doc = include_str!("readme.md")]
use core::range::Range;

/// Strongly-typed AST root node for Handlebars templates.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HandlebarsRoot {
    /// List of nodes in the template.
    pub nodes: Vec<TemplateNode>,
}

/// Enum representing a template node.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TemplateNode {
    /// Plain text content.
    Content(Content),
    /// Mustache expression.
    Mustache(Mustache),
    /// Block expression.
    Block(Block),
    /// Comment.
    Comment(Comment),
    /// Partial template reference.
    Partial(Partial),
}

/// Node representing plain text content.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Content {
    /// Text content.
    pub text: String,
    /// Span in the source code.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Node representing a Mustache expression.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Mustache {
    /// Expression content.
    pub expression: Expression,
    /// Whether the output is unescaped ({{{...}}}).
    pub is_unescaped: bool,
    /// Span in the source code.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Node representing a block expression.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Block {
    /// Block name.
    pub name: String,
    /// List of parameters.
    pub params: Vec<Expression>,
    /// Block body content.
    pub body: Vec<TemplateNode>,
    /// Inverse block content (else).
    pub inverse: Option<Vec<TemplateNode>>,
    /// Span in the source code.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Node representing a comment.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Comment {
    /// Comment text.
    pub text: String,
    /// Span in the source code.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Node representing a partial template reference.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Partial {
    /// Partial template name.
    pub name: String,
    /// List of parameters.
    pub params: Vec<Expression>,
    /// Span in the source code.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Enum representing an expression.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Expression {
    /// Path expression.
    Path(String),
    /// Literal
    Literal(Literal),
    /// Sub-expression
    SubExpression(Box<Expression>),
}

/// Literal enum
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Literal {
    /// String literal
    String(String),
    /// Number literal
    Number(f64),
    /// Boolean literal
    Boolean(bool),
}
