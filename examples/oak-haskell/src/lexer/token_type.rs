use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type HaskellToken = Token<HaskellTokenType>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HaskellTokenType {
    Whitespace,
    Newline,
    Comment,
    Case,
    Class,
    Data,
    Default,
    Deriving,
    Do,
    Else,
    Foreign,
    If,
    Import,
    In,
    Infix,
    Infixl,
    Infixr,
    Instance,
    Let,
    Module,
    Newtype,
    Of,
    Then,
    Type,
    Where,
    Underscore,
    As,
    Qualified,
    Hiding,
    Identifier,
    Constructor,
    Number,
    Integer,
    Float,
    String,
    StringLiteral,
    Char,
    CharLiteral,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Assign,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
    Arrow,
    LeftArrow,
    DoubleArrow,
    Pipe,
    Ampersand,
    Bang,
    Exclamation,
    Question,
    Colon,
    DoubleColon,
    Semicolon,
    Comma,
    Dot,
    DoubleDot,
    DotDot,
    Dollar,
    At,
    Tilde,
    Backslash,
    Append,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Quote,
    Backquote,
    Backtick,
    Function,
    DataDeclaration,
    ModuleDeclaration,
    Root,
    Error,
    Eof,
}

impl HaskellTokenType {
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::Case
                | Self::Class
                | Self::Data
                | Self::Default
                | Self::Deriving
                | Self::Do
                | Self::Else
                | Self::Foreign
                | Self::If
                | Self::Import
                | Self::In
                | Self::Infix
                | Self::Infixl
                | Self::Infixr
                | Self::Instance
                | Self::Let
                | Self::Module
                | Self::Newtype
                | Self::Of
                | Self::Then
                | Self::Type
                | Self::Where
                | Self::As
                | Self::Qualified
                | Self::Hiding
        )
    }
}

impl oak_core::TokenType for HaskellTokenType {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = oak_core::UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => oak_core::UniversalTokenRole::Whitespace,
            Self::Comment => oak_core::UniversalTokenRole::Comment,
            Self::Identifier | Self::Constructor => oak_core::UniversalTokenRole::Name,
            Self::Number | Self::Integer | Self::Float | Self::String | Self::StringLiteral | Self::Char | Self::CharLiteral => oak_core::UniversalTokenRole::Literal,
            _ if self.is_keyword() => oak_core::UniversalTokenRole::Keyword,
            Self::Plus
            | Self::Minus
            | Self::Star
            | Self::Slash
            | Self::Percent
            | Self::Assign
            | Self::Equal
            | Self::NotEqual
            | Self::Less
            | Self::Greater
            | Self::LessEqual
            | Self::GreaterEqual
            | Self::And
            | Self::Or
            | Self::Arrow
            | Self::LeftArrow
            | Self::DoubleArrow
            | Self::Pipe
            | Self::Ampersand
            | Self::Bang
            | Self::Exclamation
            | Self::Question
            | Self::Colon
            | Self::DoubleColon
            | Self::Dollar
            | Self::At
            | Self::Tilde
            | Self::Backslash
            | Self::Append => oak_core::UniversalTokenRole::Operator,
            Self::Semicolon
            | Self::Comma
            | Self::Dot
            | Self::DoubleDot
            | Self::DotDot
            | Self::LeftParen
            | Self::RightParen
            | Self::LeftBracket
            | Self::RightBracket
            | Self::LeftBrace
            | Self::RightBrace
            | Self::Underscore
            | Self::Quote
            | Self::Backquote
            | Self::Backtick => oak_core::UniversalTokenRole::Punctuation,
            Self::Eof => oak_core::UniversalTokenRole::Eof,
            _ => oak_core::UniversalTokenRole::None,
        }
    }

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }
}
