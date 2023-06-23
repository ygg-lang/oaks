use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type AsciiDocToken = Token<AsciiDocTokenType>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AsciiDocTokenType {
    Whitespace,
    Newline,
    Header1,
    Header2,
    Header3,
    Header4,
    Header5,
    Header6,
    BoldMarker,
    ItalicMarker,
    MonospaceMarker,
    CodeBlockMarker,
    LinkMarker,
    ListMarker,
    TableDelimiter,
    Comment,
    Text,
    LineBreak,
    PageBreak,
    Delimiter,
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    Colon,
    Comma,
    Dot,
    Eof,
    Error,
}

impl TokenType for AsciiDocTokenType {
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
