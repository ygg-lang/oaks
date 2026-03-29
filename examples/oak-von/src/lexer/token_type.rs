use oak_core::{Token, TokenType, UniversalTokenRole};

/// A token in VON.
pub type VonToken = Token<VonTokenType>;

/// Token types for the VON (Value-Oriented Notation) lexer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VonTokenType {
    /// Whitespace characters.
    Whitespace,
    /// Line breaks.
    Newline,
    /// Comments.
    Comment,
    /// End of file.
    Eof,
    /// An opening brace (`{`).
    LeftBrace,
    /// A closing brace (`}`).
    RightBrace,
    /// An opening bracket (`[`).
    LeftBracket,
    /// A closing bracket (`]`).
    RightBracket,
    /// An opening parenthesis (`(`).
    LeftParen,
    /// A closing parenthesis (`)`).
    RightParen,
    /// A comma (`,`).
    Comma,
    /// A colon (`:`).
    Colon,
    /// An equal sign (`=`).
    Eq,
    /// A string literal.
    StringLiteral,
    /// A numeric literal.
    NumberLiteral,
    /// A boolean literal.
    BoolLiteral,
    /// A null literal.
    NullLiteral,
    /// An identifier.
    Identifier,
    /// A value token.
    Value,
    /// An object token.
    Object,
    /// An array token.
    Array,
    /// An entry in an object.
    ObjectEntry,
    /// An enum token.
    Enum,
    /// An error node in the parse tree.
    ErrorNode,
    /// An error token.
    Error,
    /// The root of the document.
    Root,
    /// An element in an array.
    ArrayElement,
}

impl TokenType for VonTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}
