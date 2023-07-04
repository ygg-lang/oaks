use oak_core::{ElementType, UniversalElementRole};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Stylus element types.
pub enum StylusElementType {
    // Node types
    /// Root
    Root,
    /// Document
    Document,
    /// Rule
    Rule,
    /// Selector
    Selector,
    /// Property
    Property,
    /// Value
    Value,
    /// Block
    Block,

    // Lexical types
    /// Identifier
    Identifier,
    /// Number
    Number,
    /// String
    String,
    /// Color
    Color,
    /// Left brace
    LeftBrace,
    /// Right brace
    RightBrace,
    /// Left parenthesis
    LeftParen,
    /// Right parenthesis
    RightParen,
    /// Colon
    Colon,
    /// Semicolon
    Semicolon,
    /// Comma
    Comma,
    /// Dot
    Dot,
    /// Hash
    Hash,
    /// Ampersand
    Ampersand,
    /// Plus
    Plus,
    /// Minus
    Minus,
    /// Star
    Star,
    /// Slash
    Slash,
    /// Percent
    Percent,
    /// Equal
    Equal,
    /// Whitespace
    Whitespace,
    /// Newline
    Newline,
    /// Comment
    Comment,
    /// End of file
    Eof,
    /// Error
    Error,
}

impl ElementType for StylusElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            StylusElementType::Root => UniversalElementRole::Root,
            StylusElementType::Document => UniversalElementRole::Container,
            StylusElementType::Rule => UniversalElementRole::Statement,
            StylusElementType::Selector => UniversalElementRole::Binding,
            StylusElementType::Property => UniversalElementRole::AttributeKey,
            StylusElementType::Value => UniversalElementRole::Value,
            StylusElementType::Block => UniversalElementRole::Container,
            _ => UniversalElementRole::Value,
        }
    }
}

impl From<crate::lexer::token_type::StylusTokenType> for StylusElementType {
    fn from(token: crate::lexer::token_type::StylusTokenType) -> Self {
                match token {
            crate::lexer::token_type::StylusTokenType::Root => Self::Root,
            crate::lexer::token_type::StylusTokenType::Document => Self::Document,
            crate::lexer::token_type::StylusTokenType::Rule => Self::Rule,
            crate::lexer::token_type::StylusTokenType::Selector => Self::Selector,
            crate::lexer::token_type::StylusTokenType::Property => Self::Property,
            crate::lexer::token_type::StylusTokenType::Value => Self::Value,
            crate::lexer::token_type::StylusTokenType::Block => Self::Block,
            crate::lexer::token_type::StylusTokenType::Identifier => Self::Identifier,
            crate::lexer::token_type::StylusTokenType::Number => Self::Number,
            crate::lexer::token_type::StylusTokenType::String => Self::String,
            crate::lexer::token_type::StylusTokenType::Color => Self::Color,
            crate::lexer::token_type::StylusTokenType::LeftBrace => Self::LeftBrace,
            crate::lexer::token_type::StylusTokenType::RightBrace => Self::RightBrace,
            crate::lexer::token_type::StylusTokenType::LeftParen => Self::LeftParen,
            crate::lexer::token_type::StylusTokenType::RightParen => Self::RightParen,
            crate::lexer::token_type::StylusTokenType::Colon => Self::Colon,
            crate::lexer::token_type::StylusTokenType::Semicolon => Self::Semicolon,
            crate::lexer::token_type::StylusTokenType::Comma => Self::Comma,
            crate::lexer::token_type::StylusTokenType::Dot => Self::Dot,
            crate::lexer::token_type::StylusTokenType::Hash => Self::Hash,
            crate::lexer::token_type::StylusTokenType::Ampersand => Self::Ampersand,
            crate::lexer::token_type::StylusTokenType::Plus => Self::Plus,
            crate::lexer::token_type::StylusTokenType::Minus => Self::Minus,
            crate::lexer::token_type::StylusTokenType::Star => Self::Star,
            crate::lexer::token_type::StylusTokenType::Slash => Self::Slash,
            crate::lexer::token_type::StylusTokenType::Percent => Self::Percent,
            crate::lexer::token_type::StylusTokenType::Equal => Self::Equal,
            crate::lexer::token_type::StylusTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::StylusTokenType::Newline => Self::Newline,
            crate::lexer::token_type::StylusTokenType::Comment => Self::Comment,
            crate::lexer::token_type::StylusTokenType::Eof => Self::Eof,
            crate::lexer::token_type::StylusTokenType::Error => Self::Error,
            _ => Self::Error,
        }
    }
}
