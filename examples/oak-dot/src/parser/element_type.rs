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
                match token {
            crate::lexer::token_type::DotTokenType::Identifier => Self::Identifier,
            crate::lexer::token_type::DotTokenType::String => Self::String,
            crate::lexer::token_type::DotTokenType::Number => Self::Number,
            crate::lexer::token_type::DotTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::DotTokenType::Newline => Self::Newline,
            crate::lexer::token_type::DotTokenType::Graph => Self::Graph,
            crate::lexer::token_type::DotTokenType::Digraph => Self::Digraph,
            crate::lexer::token_type::DotTokenType::Subgraph => Self::Subgraph,
            crate::lexer::token_type::DotTokenType::Node => Self::Node,
            crate::lexer::token_type::DotTokenType::Edge => Self::Edge,
            crate::lexer::token_type::DotTokenType::Strict => Self::Strict,
            crate::lexer::token_type::DotTokenType::Arrow => Self::Arrow,
            crate::lexer::token_type::DotTokenType::Line => Self::Line,
            crate::lexer::token_type::DotTokenType::Equal => Self::Equal,
            crate::lexer::token_type::DotTokenType::Semicolon => Self::Semicolon,
            crate::lexer::token_type::DotTokenType::Comma => Self::Comma,
            crate::lexer::token_type::DotTokenType::LeftBrace => Self::LeftBrace,
            crate::lexer::token_type::DotTokenType::RightBrace => Self::RightBrace,
            crate::lexer::token_type::DotTokenType::LeftBracket => Self::LeftBracket,
            crate::lexer::token_type::DotTokenType::RightBracket => Self::RightBracket,
            crate::lexer::token_type::DotTokenType::LeftParen => Self::LeftParen,
            crate::lexer::token_type::DotTokenType::RightParen => Self::RightParen,
            crate::lexer::token_type::DotTokenType::Comment => Self::Comment,
            crate::lexer::token_type::DotTokenType::Root => Self::Root,
            crate::lexer::token_type::DotTokenType::Error => Self::Error,
            crate::lexer::token_type::DotTokenType::Eof => Self::Eof,
            _ => Self::Error,
        }
    }
}
