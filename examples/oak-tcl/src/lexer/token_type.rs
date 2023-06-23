use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type TclToken = Token<TclTokenType>;

impl TokenType for TclTokenType {
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
#[repr(u8)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TclTokenType {
    // 节点种类 (这些通常在 ElementType 中使用，但有时也作为标记)
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

impl From<TclTokenType> for UniversalElementRole {
    fn from(kind: TclTokenType) -> Self {
        match kind {
            TclTokenType::Root => UniversalElementRole::Root,
            TclTokenType::Command => UniversalElementRole::Expression,
            TclTokenType::Word | TclTokenType::SimpleWord | TclTokenType::VariableWord | TclTokenType::ScriptWord | TclTokenType::BracedWord => UniversalElementRole::Expression,
            TclTokenType::Identifier => UniversalElementRole::Name,
            TclTokenType::Number | TclTokenType::StringLiteral => UniversalElementRole::Value,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::parser::element_type::TclElementType> for TclTokenType {
    fn from(element: crate::parser::element_type::TclElementType) -> Self {
        unsafe { std::mem::transmute(element) }
    }
}
