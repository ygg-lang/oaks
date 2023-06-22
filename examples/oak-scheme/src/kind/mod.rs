use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SchemeSyntaxKind {
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

impl TokenType for SchemeSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::LineComment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }
}

impl ElementType for SchemeSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}
