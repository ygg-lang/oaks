use crate::LLvmLanguage;
use oak_core::{ElementType, Token, TokenType, UniversalElementRole, UniversalTokenRole};

use serde::{Deserialize, Serialize};

pub type LLvmToken = Token<LLvmLanguage>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LLvmSyntaxKind {
    Root,
    Identifier,
    Number,
    String,
    Comment,
    Whitespace,
    Newline,
    Error,
    Eof,
    // Added variants
    LocalVar,
    GlobalVar,
    Metadata,
    Equal,
    Comma,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Star,
    Colon,
    Keyword,
}

impl TokenType for LLvmSyntaxKind {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn role(&self) -> Self::Role {
        match self {
            Self::Comment => UniversalTokenRole::Comment,
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Number | Self::String => UniversalTokenRole::Literal,
            Self::Identifier | Self::LocalVar | Self::GlobalVar | Self::Metadata => UniversalTokenRole::Name,
            Self::Keyword => UniversalTokenRole::Keyword,
            Self::Error => UniversalTokenRole::Error,
            Self::Equal | Self::Comma | Self::LParen | Self::RParen | Self::LBracket | Self::RBracket | Self::LBrace | Self::RBrace | Self::Star | Self::Colon => UniversalTokenRole::Operator,
            _ => UniversalTokenRole::None,
        }
    }

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }
}

impl ElementType for LLvmSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}
