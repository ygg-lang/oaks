use oak_core::{ElementType, Parser, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AsciiDocElementType {
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
    Root,
}

impl ElementType for AsciiDocElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::AsciiDocTokenType> for AsciiDocElementType {
    fn from(token: crate::lexer::token_type::AsciiDocTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
