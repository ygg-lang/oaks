use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type SchemeToken = Token<SchemeTokenType>;

impl TokenType for SchemeTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment | Self::LineComment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment | Self::LineComment => UniversalTokenRole::Comment,
            Self::Error => UniversalTokenRole::Error,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SchemeTokenType {
    // 空白字符和换行
    Whitespace,
    Newline,
    Comment,

    // 注释
    LineComment,

    // 字面量
    NumberLiteral,
    StringLiteral,
    CharacterLiteral,
    BooleanLiteral,

    // 标识符和符号
    Identifier,
    Symbol,

    // 关键字
    Keyword,
    Define,
    Lambda,
    If,
    Cond,
    Case,
    Let,
    LetStar,
    Letrec,
    Begin,
    Do,
    Quote,
    Quasiquote,
    Unquote,
    UnquoteSplicing,
    And,
    Or,
    Not,
    Set,

    // 分隔符
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Dot,

    // 特殊符号
    Hash,
    Quote_,
    Quasiquote_,
    Unquote_,
    UnquoteSplicing_,

    // 错误和结束
    Error,
    Eof,

    // 根节点
    SourceFile,
}
