use oak_core::{Token, TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type LuaToken = Token<LuaTokenType>;

impl TokenType for LuaTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::EndOfStream;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::And
            | Self::Break
            | Self::Do
            | Self::Else
            | Self::Elseif
            | Self::End
            | Self::False
            | Self::For
            | Self::Function
            | Self::Goto
            | Self::If
            | Self::In
            | Self::Local
            | Self::Nil
            | Self::Not
            | Self::Or
            | Self::Repeat
            | Self::Return
            | Self::Then
            | Self::True
            | Self::Until
            | Self::While => UniversalTokenRole::Keyword,

            Self::Identifier => UniversalTokenRole::Name,
            Self::Number => UniversalTokenRole::Literal,
            Self::String => UniversalTokenRole::Literal,

            Self::Plus
            | Self::Minus
            | Self::Star
            | Self::Slash
            | Self::Percent
            | Self::Caret
            | Self::Hash
            | Self::Ampersand
            | Self::Tilde
            | Self::Pipe
            | Self::LtLt
            | Self::GtGt
            | Self::SlashSlash
            | Self::EqEq
            | Self::TildeEq
            | Self::LtEq
            | Self::GtEq
            | Self::Lt
            | Self::Gt
            | Self::Eq
            | Self::DotDot
            | Self::DotDotDot => UniversalTokenRole::Operator,

            Self::LeftParen | Self::RightParen | Self::LeftBrace | Self::RightBrace | Self::LeftBracket | Self::RightBracket | Self::ColonColon | Self::Semicolon | Self::Colon | Self::Comma | Self::Dot => UniversalTokenRole::Punctuation,

            Self::Comment => UniversalTokenRole::Comment,
            _ => UniversalTokenRole::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u16)]
pub enum LuaTokenType {
    Root,
    // 关键字
    And,
    Break,
    Do,
    Else,
    Elseif,
    End,
    False,
    For,
    Function,
    Goto,
    If,
    In,
    Local,
    Nil,
    Not,
    Or,
    Repeat,
    Return,
    Then,
    True,
    Until,
    While,

    // 标识符和字面量
    Identifier,
    Number,
    String,

    // 操作符
    Plus,       // +
    Minus,      // -
    Star,       // *
    Slash,      // /
    Percent,    // %
    Caret,      // ^
    Hash,       // #
    Ampersand,  // &
    Tilde,      // ~
    Pipe,       // |
    LtLt,       // <<
    GtGt,       // >>
    SlashSlash, // //
    EqEq,       // ==
    TildeEq,    // ~=
    LtEq,       // <=
    GtEq,       // >=
    Lt,         // <
    Gt,         // >
    Eq,         // =

    // 分隔符
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    ColonColon,   // ::
    Semicolon,    // ;
    Colon,        // :
    Comma,        // ,
    Dot,          // .
    DotDot,       // ..
    DotDotDot,    // ...

    // 空白和注释
    Whitespace,
    Newline,
    Comment,

    // 特殊标记
    EndOfStream,
    Error,
}
