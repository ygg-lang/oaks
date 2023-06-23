use oak_core::{TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Ada 词法单元类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AdaTokenType {
    Whitespace,
    Newline,
    Comment,

    StringLiteral,
    CharacterLiteral,
    NumberLiteral,
    Identifier,

    Abort,
    Abs,
    Abstract,
    Accept,
    Access,
    Aliased,
    All,
    And,
    Array,
    At,
    Begin,
    Body,
    Case,
    Constant,
    Declare,
    Delay,
    Delta,
    Digits,
    Do,
    Else,
    Elsif,
    End,
    Entry,
    Exception,
    Exit,
    For,
    Function,
    Generic,
    Goto,
    If,
    In,
    Interface,
    Is,
    Limited,
    Loop,
    Mod,
    New,
    Not,
    Null,
    Of,
    Or,
    Others,
    Out,
    Overriding,
    Package,
    Pragma,
    Private,
    Procedure,
    Protected,
    Raise,
    Range,
    Record,
    Rem,
    Renames,
    Requeue,
    Return,
    Reverse,
    Select,
    Separate,
    Some,
    Subtype,
    Synchronized,
    Tagged,
    Task,
    Terminate,
    Then,
    Type,
    Until,
    Use,
    When,
    While,
    With,
    Xor,

    // 符号
    Plus,         // +
    Minus,        // -
    Star,         // *
    Slash,        // /
    Ampersand,    // &
    Eq,           // =
    Ne,           // /=
    Lt,           // <
    Le,           // <=
    Gt,           // >
    Ge,           // >=
    Assign,       // :=
    ColonEq,      // := (alias)
    Arrow,        // =>
    Dot,          // .
    DotDot,       // ..
    Comma,        // ,
    Colon,        // :
    Semicolon,    // ;
    Bar,          // |
    Pipe,         // | (alias)
    Apostrophe,   // '
    Tick,         // ' (alias)
    LeftParen,    // (
    RightParen,   // )
    Box,          // <>
    DoubleStar,   // **
    StarStar,     // ** (alias)
    LtLt,         // <<
    GtGt,         // >>
    LeftBracket,  // [
    RightBracket, // ]
    LeftBrace,    // {
    RightBrace,   // }

    Eof,
    Error,
}

impl AdaTokenType {
    /// 是否为关键字
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::Abort
                | Self::Abs
                | Self::Abstract
                | Self::Accept
                | Self::Access
                | Self::Aliased
                | Self::All
                | Self::And
                | Self::Array
                | Self::At
                | Self::Begin
                | Self::Body
                | Self::Case
                | Self::Constant
                | Self::Declare
                | Self::Delay
                | Self::Delta
                | Self::Digits
                | Self::Do
                | Self::Else
                | Self::Elsif
                | Self::End
                | Self::Entry
                | Self::Exception
                | Self::Exit
                | Self::For
                | Self::Function
                | Self::Generic
                | Self::Goto
                | Self::If
                | Self::In
                | Self::Interface
                | Self::Is
                | Self::Limited
                | Self::Loop
                | Self::Mod
                | Self::New
                | Self::Not
                | Self::Null
                | Self::Of
                | Self::Or
                | Self::Others
                | Self::Out
                | Self::Overriding
                | Self::Package
                | Self::Pragma
                | Self::Private
                | Self::Procedure
                | Self::Protected
                | Self::Raise
                | Self::Range
                | Self::Record
                | Self::Rem
                | Self::Renames
                | Self::Requeue
                | Self::Return
                | Self::Reverse
                | Self::Select
                | Self::Separate
                | Self::Some
                | Self::Subtype
                | Self::Synchronized
                | Self::Tagged
                | Self::Task
                | Self::Terminate
                | Self::Then
                | Self::Type
                | Self::Until
                | Self::Use
                | Self::When
                | Self::While
                | Self::With
                | Self::Xor
        )
    }

    /// 是否为标识符
    pub fn is_identifier(&self) -> bool {
        matches!(self, Self::Identifier)
    }

    /// 是否为字面量
    pub fn is_literal(&self) -> bool {
        matches!(self, Self::StringLiteral | Self::CharacterLiteral | Self::NumberLiteral)
    }
}

impl TokenType for AdaTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    fn role(&self) -> Self::Role {
        use UniversalTokenRole::*;
        match self {
            _ if self.is_keyword() => Keyword,
            Self::Identifier => Name,
            _ if self.is_literal() => Literal,
            Self::Comment => Comment,
            Self::Whitespace | Self::Newline => Whitespace,
            Self::Error => Error,
            Self::Eof => Eof,
            Self::Plus
            | Self::Minus
            | Self::Star
            | Self::Slash
            | Self::Ampersand
            | Self::Eq
            | Self::Ne
            | Self::Lt
            | Self::Le
            | Self::Gt
            | Self::Ge
            | Self::Assign
            | Self::ColonEq
            | Self::Arrow
            | Self::DoubleStar
            | Self::StarStar
            | Self::LtLt
            | Self::GtGt => Operator,
            Self::Dot
            | Self::DotDot
            | Self::Comma
            | Self::Colon
            | Self::Semicolon
            | Self::Bar
            | Self::Pipe
            | Self::Apostrophe
            | Self::Tick
            | Self::LeftParen
            | Self::RightParen
            | Self::Box
            | Self::LeftBracket
            | Self::RightBracket
            | Self::LeftBrace
            | Self::RightBrace => Punctuation,
            _ => None,
        }
    }
}
