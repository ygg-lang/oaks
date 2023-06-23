use oak_core::{Token, TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type ScalaToken = Token<ScalaTokenType>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ScalaTokenType {
    // Node kinds
    SourceFile,

    // Basic token kinds
    Whitespace,
    Newline,
    Comment,
    LineComment,
    BlockComment,
    Error,
    Eof,
    ErrorNode,

    // Identifiers and literals
    Identifier,
    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
    CharLiteral,
    BooleanLiteral,

    // Keywords
    Abstract,
    Case,
    Catch,
    Class,
    Def,
    Do,
    Else,
    Extends,
    False,
    Final,
    Finally,
    For,
    ForSome,
    If,
    Implicit,
    Import,
    Lazy,
    Match,
    New,
    Null,
    Object,
    Override,
    Package,
    Private,
    Protected,
    Return,
    Sealed,
    Super,
    This,
    Throw,
    Trait,
    Try,
    True,
    Type,
    Val,
    Var,
    While,
    With,
    Yield,

    // Operators
    Plus,         // +
    Minus,        // -
    Star,         // *
    Slash,        // /
    Percent,      // %
    Eq,           // =
    EqEq,         // ==
    Ne,           // !=
    Lt,           // <
    Le,           // <=
    Gt,           // >
    Ge,           // >=
    LessEqual,    // <=
    GreaterEqual, // >=
    EqualEqual,   // ==
    NotEqual,     // !=
    And,          // &
    Or,           // |
    Xor,          // ^
    AndAnd,       // &&
    OrOr,         // ||
    Not,          // !
    Tilde,        // ~
    LShift,       // <<
    RShift,       // >>
    URShift,      // >>>
    PlusEq,       // +=
    MinusEq,      // -=
    StarEq,       // *=
    SlashEq,      // /=
    PercentEq,    // %=
    AndEq,        // &=
    OrEq,         // |=
    XorEq,        // ^=
    LShiftEq,     // <<=
    RShiftEq,     // >>=
    URShiftEq,    // >>>=
    Arrow,        // =>
    LeftArrow,    // <-
    Colon,        // :
    ColonColon,   // ::
    Semicolon,    // ;
    Dot,          // .
    Comma,        // ,
    Question,     // ?
    At,           // @
    Hash,         // #

    // 分隔符
    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    LeftBrace,    // {
    RightBrace,   //
}

impl TokenType for ScalaTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}
