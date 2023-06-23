use oak_core::{ElementType, Parser, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TclElementType {
    // 节点种类
    Root,
    Command,
    Word,
    SimpleWord,
    VariableWord,
    ScriptWord,
    BracedWord,

    // 字面量
    Number,
    StringLiteral,
    Identifier,

    // 关键字
    If,
    Else,
    ElseIf,
    For,
    While,
    ForEach,
    Proc,
    Return,
    Break,
    Continue,
    Set,
    Unset,
    Global,
    Upvar,
    Variable,

    // 操作符
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Ampersand,
    AmpersandAmpersand,
    Pipe,
    PipePipe,
    Exclamation,

    // 标点符号
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Semicolon,
    Comma,
    Dollar,

    // 特殊
    Whitespace,
    Newline,
    Comment,
    Error,
    Eof,
}

impl ElementType for TclElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            TclElementType::Root => UniversalElementRole::Root,
            TclElementType::Command => UniversalElementRole::Expression,
            TclElementType::Word | TclElementType::SimpleWord | TclElementType::VariableWord | TclElementType::ScriptWord | TclElementType::BracedWord => UniversalElementRole::Expression,
            TclElementType::Identifier => UniversalElementRole::Name,
            TclElementType::Number | TclElementType::StringLiteral => UniversalElementRole::Value,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::TclTokenType> for TclElementType {
    fn from(token: crate::lexer::token_type::TclTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
