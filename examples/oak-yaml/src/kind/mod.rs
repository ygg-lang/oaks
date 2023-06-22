use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u16)]
pub enum YamlSyntaxKind {
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

impl TokenType for YamlSyntaxKind {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn role(&self) -> Self::Role {
        use oak_core::UniversalTokenRole::*;
        match self {
            Self::Colon
            | Self::Dash
            | Self::Pipe
            | Self::GreaterThan
            | Self::Question
            | Self::Ampersand
            | Self::Asterisk
            | Self::Exclamation
            | Self::LeftBracket
            | Self::RightBracket
            | Self::LeftBrace
            | Self::RightBrace
            | Self::DocumentStart
            | Self::DocumentEnd => Punctuation,

            Self::Identifier | Self::Anchor | Self::Alias | Self::Tag => Name,

            Self::StringLiteral | Self::NumberLiteral | Self::BooleanLiteral | Self::NullLiteral => Literal,

            Self::Whitespace | Self::Newline => Whitespace,
            Self::Comment => Comment,
            Self::Error => Error,
            _ => None,
        }
    }
}

impl ElementType for YamlSyntaxKind {
    type Role = UniversalElementRole;

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}
