use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::LLvmLanguage;

pub type LLvmToken = Token<LLvmLanguage>;

/// LLVM IR token types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    const END_OF_STREAM: Self = Self::Error;

    fn is_ignored(&self) -> bool {
        match self {
            Self::Whitespace | Self::Comment => true,
            _ => false,
        }
    }

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalTokenRole::None,
        }
    }
}
