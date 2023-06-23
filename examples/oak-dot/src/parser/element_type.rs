use oak_core::{ElementType, Parser, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum DotElementType {
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

impl ElementType for DotElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::DotTokenType> for DotElementType {
    fn from(token: crate::lexer::token_type::DotTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
