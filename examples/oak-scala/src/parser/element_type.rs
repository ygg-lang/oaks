use oak_core::{ElementType, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ScalaElementType {
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

impl ElementType for ScalaElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::SourceFile => UniversalElementRole::Root,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::ScalaTokenType> for ScalaElementType {
    fn from(token: crate::lexer::token_type::ScalaTokenType) -> Self {
        use crate::lexer::token_type::ScalaTokenType as T;
        match token {
            T::SourceFile => ScalaElementType::SourceFile,
            T::Whitespace => ScalaElementType::Whitespace,
            T::Newline => ScalaElementType::Newline,
            T::Comment => ScalaElementType::Comment,
            T::LineComment => ScalaElementType::LineComment,
            T::BlockComment => ScalaElementType::BlockComment,
            T::Error => ScalaElementType::Error,
            T::Eof => ScalaElementType::Eof,
            T::ErrorNode => ScalaElementType::ErrorNode,
            T::Identifier => ScalaElementType::Identifier,
            T::IntegerLiteral => ScalaElementType::IntegerLiteral,
            T::FloatLiteral => ScalaElementType::FloatLiteral,
            T::StringLiteral => ScalaElementType::StringLiteral,
            T::CharLiteral => ScalaElementType::CharLiteral,
            T::BooleanLiteral => ScalaElementType::BooleanLiteral,
            T::Abstract => ScalaElementType::Abstract,
            T::Case => ScalaElementType::Case,
            T::Catch => ScalaElementType::Catch,
            T::Class => ScalaElementType::Class,
            T::Def => ScalaElementType::Def,
            T::Do => ScalaElementType::Do,
            T::Else => ScalaElementType::Else,
            T::Extends => ScalaElementType::Extends,
            T::False => ScalaElementType::False,
            T::Final => ScalaElementType::Final,
            T::Finally => ScalaElementType::Finally,
            T::For => ScalaElementType::For,
            T::ForSome => ScalaElementType::ForSome,
            T::If => ScalaElementType::If,
            T::Implicit => ScalaElementType::Implicit,
            T::Import => ScalaElementType::Import,
            T::Lazy => ScalaElementType::Lazy,
            T::Match => ScalaElementType::Match,
            T::New => ScalaElementType::New,
            T::Null => ScalaElementType::Null,
            T::Object => ScalaElementType::Object,
            T::Override => ScalaElementType::Override,
            T::Package => ScalaElementType::Package,
            T::Private => ScalaElementType::Private,
            T::Protected => ScalaElementType::Protected,
            T::Return => ScalaElementType::Return,
            T::Sealed => ScalaElementType::Sealed,
            T::Super => ScalaElementType::Super,
            T::This => ScalaElementType::This,
            T::Throw => ScalaElementType::Throw,
            T::Trait => ScalaElementType::Trait,
            T::Try => ScalaElementType::Try,
            T::True => ScalaElementType::True,
            T::Type => ScalaElementType::Type,
            T::Val => ScalaElementType::Val,
            T::Var => ScalaElementType::Var,
            T::While => ScalaElementType::While,
            T::With => ScalaElementType::With,
            T::Yield => ScalaElementType::Yield,
            T::Plus => ScalaElementType::Plus,
            T::Minus => ScalaElementType::Minus,
            T::Star => ScalaElementType::Star,
            T::Slash => ScalaElementType::Slash,
            T::Percent => ScalaElementType::Percent,
            T::Eq => ScalaElementType::Eq,
            T::EqEq => ScalaElementType::EqEq,
            T::Ne => ScalaElementType::Ne,
            T::Lt => ScalaElementType::Lt,
            T::Le => ScalaElementType::Le,
            T::Gt => ScalaElementType::Gt,
            T::Ge => ScalaElementType::Ge,
            T::LessEqual => ScalaElementType::LessEqual,
            T::GreaterEqual => ScalaElementType::GreaterEqual,
            T::EqualEqual => ScalaElementType::EqualEqual,
            T::NotEqual => ScalaElementType::NotEqual,
            T::And => ScalaElementType::And,
            T::Or => ScalaElementType::Or,
            T::Xor => ScalaElementType::Xor,
            T::AndAnd => ScalaElementType::AndAnd,
            T::OrOr => ScalaElementType::OrOr,
            T::Not => ScalaElementType::Not,
            T::Tilde => ScalaElementType::Tilde,
            T::LShift => ScalaElementType::LShift,
            T::RShift => ScalaElementType::RShift,
            T::URShift => ScalaElementType::URShift,
            T::PlusEq => ScalaElementType::PlusEq,
            T::MinusEq => ScalaElementType::MinusEq,
            T::StarEq => ScalaElementType::StarEq,
            T::SlashEq => ScalaElementType::SlashEq,
            T::PercentEq => ScalaElementType::PercentEq,
            T::AndEq => ScalaElementType::AndEq,
            T::OrEq => ScalaElementType::OrEq,
            T::XorEq => ScalaElementType::XorEq,
            T::LShiftEq => ScalaElementType::LShiftEq,
            T::RShiftEq => ScalaElementType::RShiftEq,
            T::URShiftEq => ScalaElementType::URShiftEq,
            T::Arrow => ScalaElementType::Arrow,
            T::LeftArrow => ScalaElementType::LeftArrow,
            T::Colon => ScalaElementType::Colon,
            T::ColonColon => ScalaElementType::ColonColon,
            T::Semicolon => ScalaElementType::Semicolon,
            T::Dot => ScalaElementType::Dot,
            T::Comma => ScalaElementType::Comma,
            T::Question => ScalaElementType::Question,
            T::At => ScalaElementType::At,
            T::Hash => ScalaElementType::Hash,
            T::LeftParen => ScalaElementType::LeftParen,
            T::RightParen => ScalaElementType::RightParen,
            T::LeftBracket => ScalaElementType::LeftBracket,
            T::RightBracket => ScalaElementType::RightBracket,
            T::LeftBrace => ScalaElementType::LeftBrace,
            T::RightBrace => ScalaElementType::RightBrace,
        }
    }
}
