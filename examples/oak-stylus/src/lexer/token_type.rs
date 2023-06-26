use oak_core::{Token, TokenType, UniversalTokenRole};

/// Token type for Stylus lexer output.
pub type StylusToken = Token<StylusTokenType>;

impl TokenType for StylusTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn role(&self) -> Self::Role {
        match self {
            Self::Identifier => UniversalTokenRole::Name,
            Self::Number | Self::String | Self::Color => UniversalTokenRole::Literal,
            Self::LeftBrace | Self::RightBrace | Self::LeftParen | Self::RightParen | Self::Colon | Self::Semicolon | Self::Comma => UniversalTokenRole::Punctuation,
            Self::Dot | Self::Hash | Self::Ampersand | Self::Plus | Self::Minus | Self::Star | Self::Slash | Self::Percent | Self::Equal => UniversalTokenRole::Operator,
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}

impl StylusTokenType {
    /// Checks if it is a value type.
    pub fn is_value(self) -> bool {
        matches!(self, StylusTokenType::Number | StylusTokenType::String | StylusTokenType::Color | StylusTokenType::Identifier)
    }

    /// Checks if it is an operator.
    pub fn is_operator(self) -> bool {
        matches!(self, StylusTokenType::Plus | StylusTokenType::Minus | StylusTokenType::Star | StylusTokenType::Slash | StylusTokenType::Percent | StylusTokenType::Equal)
    }
}

impl core::fmt::Display for StylusTokenType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let name = match self {
            StylusTokenType::Root => "Root",
            StylusTokenType::Document => "Document",
            StylusTokenType::Rule => "Rule",
            StylusTokenType::Selector => "Selector",
            StylusTokenType::Property => "Property",
            StylusTokenType::Value => "Value",
            StylusTokenType::Block => "Block",
            StylusTokenType::Identifier => "Identifier",
            StylusTokenType::Number => "Number",
            StylusTokenType::String => "String",
            StylusTokenType::Color => "Color",
            StylusTokenType::LeftBrace => "LeftBrace",
            StylusTokenType::RightBrace => "RightBrace",
            StylusTokenType::LeftParen => "LeftParen",
            StylusTokenType::RightParen => "RightParen",
            StylusTokenType::Colon => "Colon",
            StylusTokenType::Semicolon => "Semicolon",
            StylusTokenType::Comma => "Comma",
            StylusTokenType::Dot => "Dot",
            StylusTokenType::Hash => "Hash",
            StylusTokenType::Ampersand => "Ampersand",
            StylusTokenType::Plus => "Plus",
            StylusTokenType::Minus => "Minus",
            StylusTokenType::Star => "Star",
            StylusTokenType::Slash => "Slash",
            StylusTokenType::Percent => "Percent",
            StylusTokenType::Equal => "Equal",
            StylusTokenType::Whitespace => "Whitespace",
            StylusTokenType::Newline => "Newline",
            StylusTokenType::Comment => "Comment",
            StylusTokenType::Eof => "Eof",
            StylusTokenType::Error => "Error",
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Stylus token types.
pub enum StylusTokenType {
    // Node types
    /// Root node
    Root,
    /// Document node
    Document,
    /// Rule node
    Rule,
    /// Selector node
    Selector,
    /// Property node
    Property,
    /// Value node
    Value,
    /// Block node
    Block,

    // Lexical types
    /// Identifier (body, div, color, etc.)
    Identifier,
    /// Number (10, 100px, 1.5em)
    Number,
    /// String ("Arial", 'Helvetica')
    String,
    /// Color (#fff, red, rgb(255,0,0))
    Color,
    /// Left brace {
    LeftBrace,
    /// Right brace }
    RightBrace,
    /// Left parenthesis (
    LeftParen,
    /// Right parenthesis )
    RightParen,
    /// Colon :
    Colon,
    /// Semicolon ;
    Semicolon,
    /// Comma ,
    Comma,
    /// Dot .
    Dot,
    /// Hash #
    Hash,
    /// Ampersand &
    Ampersand,
    /// Plus +
    Plus,
    /// Minus -
    Minus,
    /// Star *
    Star,
    /// Slash /
    Slash,
    /// Percent %
    Percent,
    /// Equal =
    Equal,
    /// Whitespace
    Whitespace,
    /// Newline
    Newline,
    /// Comment
    Comment,
    /// End of file
    Eof,
    /// Error token
    Error,
}
