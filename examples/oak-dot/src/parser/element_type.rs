//! Element types for the DOT language.
use oak_core::{ElementType, UniversalElementRole};

/// Element types for the DOT language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum DotElementType {
    // Basic kind
    /// An identifier.
    Identifier,
    /// A string literal.
    String,
    /// A number literal.
    Number,
    /// Whitespace.
    Whitespace,
    /// A newline.
    Newline,

    // DOT keywords
    /// The `graph` keyword.
    Graph,
    /// The `digraph` keyword.
    Digraph,
    /// The `subgraph` keyword.
    Subgraph,
    /// The `node` keyword.
    Node,
    /// The `edge` keyword.
    Edge,
    /// The `strict` keyword.
    Strict,

    // Operators
    /// The `->` arrow operator.
    Arrow, // ->
    /// The `--` line operator.
    Line, // --
    /// The `=` equal operator.
    Equal, // =
    /// The `;` semicolon.
    Semicolon, // ;
    /// The `,` comma.
    Comma, // ,

    // Delimiters
    /// The `{` left brace.
    LeftBrace, // {
    /// The `}` right brace.
    RightBrace, // }
    /// The `[` left bracket.
    LeftBracket, // [
    /// The `]` right bracket.
    RightBracket, // ]
    /// The `(` left paren.
    LeftParen, // (
    /// The `)` right paren.
    RightParen, // )

    // Comments
    /// A comment.
    Comment,

    // Special
    /// The root element.
    Root,
    /// An error element.
    Error,
    /// End of stream marker.
    Eof,
}

impl ElementType for DotElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::DotTokenType> for DotElementType {
    fn from(token: crate::lexer::token_type::DotTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
