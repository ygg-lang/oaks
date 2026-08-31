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
        unsafe { std::mem::transmute(token) }
    }
}
