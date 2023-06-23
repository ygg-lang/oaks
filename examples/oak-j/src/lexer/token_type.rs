use oak_core::{TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// J 词法单元类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum JTokenType {
    Whitespace,
    Newline,
    Comment,

    StringLiteral,
    NumberLiteral,
    Identifier,

    // J 原始操作符 (Verbs, Adverbs, Conjunctions)
    // 基础符号
    Equal, // =
    Dot,   // .
    Colon, // :

    // 赋值
    IsGlobal, // =:
    IsLocal,  // =.

    // 常用动词
    Plus,      // +
    Minus,     // -
    Star,      // *
    Percent,   // %
    Dollar,    // $
    Comma,     // ,
    Hash,      // #
    Slash,     // /
    Backslash, // \
    Pipe,      // |
    Ampersand, // &
    Caret,     // ^
    Tilde,     // ~
    Less,      // <
    Greater,   // >

    // 括号
    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    LeftBrace,    // {
    RightBrace,   // }

    // 特殊
    Eof,
    Error,
}

impl JTokenType {
    /// 是否为关键字 (J 中更多是原始操作符)
    pub fn is_keyword(&self) -> bool {
        false
    }

    /// 是否为标点符号
    pub fn is_punctuation(&self) -> bool {
        matches!(self, Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::LeftBrace | Self::RightBrace)
    }

    /// 是否为忽略的单元 (空白或注释)
    pub fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }
}

impl TokenType for JTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::StringLiteral | Self::NumberLiteral => UniversalTokenRole::Literal,
            Self::Identifier => UniversalTokenRole::Name,
            _ if self.is_punctuation() => UniversalTokenRole::Punctuation,
            Self::Eof | Self::Error => UniversalTokenRole::None,
            _ => UniversalTokenRole::Operator,
        }
    }
}
