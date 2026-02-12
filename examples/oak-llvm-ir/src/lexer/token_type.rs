use oak_core::{Token, TokenType, UniversalTokenRole};

use crate::LLvmLanguage;

/// A token in an LLVM IR source file.
pub type LLvmToken = Token<LLvmLanguage>;

/// LLVM IR token types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum LLvmTokenType {
    /// Root node.
    Root,
    /// Identifier.
    Identifier,
    /// Numeric literal.
    Number,
    /// String literal.
    String,
    /// Comment.
    Comment,
    /// Whitespace.
    Whitespace,
    /// Newline.
    Newline,
    /// Error token.
    Error,
    /// End of stream.
    Eof,
    // Added variants
    /// Local variable (starts with %).
    LocalVar,
    /// Global variable (starts with @).
    GlobalVar,
    /// Metadata.
    Metadata,
    /// Equal sign (=).
    Equal,
    /// Comma (,).
    Comma,
    /// Left parenthesis (().
    LParen,
    /// Right parenthesis ()).
    RParen,
    /// Left bracket ([).
    LBracket,
    /// Right bracket (]).
    RBracket,
    /// Left brace ({).
    LBrace,
    /// Right brace (}).
    RBrace,
    /// Star (*).
    Star,
    /// Colon (:).
    Colon,
    /// Keyword.
    Keyword,
}

impl TokenType for LLvmTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        match self {
            Self::Whitespace | Self::Comment | Self::Newline => true,
            _ => false,
        }
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Keyword => UniversalTokenRole::Keyword,
            Self::Number | Self::String => UniversalTokenRole::Literal,
            Self::Equal | Self::Star => UniversalTokenRole::Operator,
            Self::Comma | Self::LParen | Self::RParen | Self::LBracket | Self::RBracket | Self::LBrace | Self::RBrace | Self::Colon => UniversalTokenRole::Punctuation,
            Self::LocalVar | Self::GlobalVar | Self::Metadata | Self::Identifier => UniversalTokenRole::Name,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}
