use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type ClojureToken = Token<ClojureTokenType>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ClojureTokenType {
    Token,
    List,
    Vector,
    Map,
    Set,
    AnonFn,
    Root,
    SourceFile,
    Error,
    // Lexer tokens
    ListStart,
    ListEnd,
    VectorStart,
    VectorEnd,
    MapStart,
    MapEnd,
    SetStart,
    AnonFnStart,
    Quote,
    Unquote,
    UnquoteSplice,
    Meta,
    Whitespace,
    Comment,
    StringLiteral,
    CharacterLiteral,
    NumberLiteral,
    KeywordLiteral,
    Dispatch,
    RegexLiteral,
    Symbol,
}

impl TokenType for ClojureTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Error;

    fn is_ignored(&self) -> bool {
        false
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}
