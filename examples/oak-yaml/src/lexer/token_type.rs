use oak_core::{Token, TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type YamlToken = Token<YamlTokenType>;

impl TokenType for YamlTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::StringLiteral | Self::NumberLiteral | Self::BooleanLiteral | Self::NullLiteral => UniversalTokenRole::Literal,
            Self::Identifier | Self::Anchor | Self::Alias | Self::Tag => UniversalTokenRole::Name,
            Self::Colon | Self::Dash | Self::Pipe | Self::GreaterThan | Self::Question | Self::Ampersand | Self::Asterisk | Self::Exclamation => UniversalTokenRole::Operator,
            Self::LeftBracket | Self::RightBracket | Self::LeftBrace | Self::RightBrace | Self::DocumentStart | Self::DocumentEnd => UniversalTokenRole::Punctuation,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u16)]
pub enum YamlTokenType {
    // Trivia
    Whitespace,
    Comment,

    // Literals
    StringLiteral,
    NumberLiteral,
    BooleanLiteral,
    NullLiteral,

    // Identifiers
    Identifier,

    // Operators and punctuation
    Colon,       // :
    Dash,        // -
    Pipe,        // |
    GreaterThan, // >
    Question,    // ?
    Ampersand,   // &
    Asterisk,    // *
    Exclamation, // !

    // Brackets
    LeftBracket,  // [
    RightBracket, // ]
    LeftBrace,    // {
    RightBrace,   // }

    // Special
    Anchor, // &anchor
    Alias,  // *alias
    Tag,    // !tag

    // Document markers
    DocumentStart, // ---
    DocumentEnd,   // ...
    Document,
    Root,

    // Newlines and indentation
    Newline,

    // Error and EOF
    Error,
    Eof,
}
