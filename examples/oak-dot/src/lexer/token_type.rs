use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type DotToken = Token<DotTokenType>;

impl TokenType for DotTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Error;

    fn is_ignored(&self) -> bool {
        false
    }

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalTokenRole::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum DotTokenType {
    // 基本 kind
    Identifier,
    String,
    Number,
    Whitespace,
    Newline,

    // DOT 关键字
    Graph,
    Digraph,
    Subgraph,
    Node,
    Edge,
    Strict,

    // 操作符
    Arrow,     // ->
    Line,      // --
    Equal,     // =
    Semicolon, // ;
    Comma,     // ,

    // 分隔符
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    LeftParen,    // (
    RightParen,   // )

    // 注释
    Comment,

    // 特殊
    Root,
    Error,
    Eof,
}
