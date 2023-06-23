use oak_core::{Token, TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type DelphiToken = Token<DelphiTokenType>;

impl From<crate::parser::element_type::DelphiElementType> for DelphiTokenType {
    fn from(element: crate::parser::element_type::DelphiElementType) -> Self {
        use crate::parser::element_type::DelphiElementType as E;
        match element {
            E::Root => Self::Root,
            E::Identifier => Self::Identifier,
            E::String => Self::String,
            E::Number => Self::Number,
            E::Float => Self::Float,
            E::Whitespace => Self::Whitespace,
            E::Newline => Self::Newline,
            E::Program => Self::Program,
            E::Unit => Self::Unit,
            E::Interface => Self::Interface,
            E::Implementation => Self::Implementation,
            E::Uses => Self::Uses,
            E::Type => Self::Type,
            E::Var => Self::Var,
            E::Const => Self::Const,
            E::Function => Self::Function,
            E::Procedure => Self::Procedure,
            E::Begin => Self::Begin,
            E::End => Self::End,
            E::If => Self::If,
            E::Then => Self::Then,
            E::Else => Self::Else,
            E::While => Self::While,
            E::Do => Self::Do,
            E::For => Self::For,
            E::To => Self::To,
            E::Downto => Self::Downto,
            E::Repeat => Self::Repeat,
            E::Until => Self::Until,
            E::Case => Self::Case,
            E::Of => Self::Of,
            E::With => Self::With,
            E::Try => Self::Try,
            E::Except => Self::Except,
            E::Finally => Self::Finally,
            E::Raise => Self::Raise,
            E::Class => Self::Class,
            E::Object => Self::Object,
            E::Record => Self::Record,
            E::Array => Self::Array,
            E::Set => Self::Set,
            E::File => Self::File,
            E::Packed => Self::Packed,
            E::String_ => Self::String_,
            E::Integer => Self::Integer,
            E::Real => Self::Real,
            E::Boolean => Self::Boolean,
            E::Char => Self::Char,
            E::Pointer => Self::Pointer,
            E::Nil => Self::Nil,
            E::True_ => Self::True_,
            E::False_ => Self::False_,
            E::And_ => Self::And_,
            E::Or_ => Self::Or_,
            E::Not_ => Self::Not_,
            E::Div => Self::Div,
            E::Mod => Self::Mod,
            E::In_ => Self::In_,
            E::Is_ => Self::Is_,
            E::As_ => Self::As_,
            E::Plus => Self::Plus,
            E::Minus => Self::Minus,
            E::Star => Self::Star,
            E::Slash => Self::Slash,
            E::Equal => Self::Equal,
            E::NotEqual => Self::NotEqual,
            E::Less => Self::Less,
            E::Greater => Self::Greater,
            E::LessEqual => Self::LessEqual,
            E::GreaterEqual => Self::GreaterEqual,
            E::Assign => Self::Assign,
            E::Dot => Self::Dot,
            E::DotDot => Self::DotDot,
            E::Caret => Self::Caret,
            E::At => Self::At,
            E::LeftParen => Self::LeftParen,
            E::RightParen => Self::RightParen,
            E::LeftBracket => Self::LeftBracket,
            E::RightBracket => Self::RightBracket,
            E::Semicolon => Self::Semicolon,
            E::Comma => Self::Comma,
            E::Colon => Self::Colon,
            E::Comment => Self::Comment,
            E::LineComment => Self::LineComment,
            E::BlockComment => Self::BlockComment,
            E::Error => Self::Error,
            E::Eof => Self::Eof,
        }
    }
}

impl DelphiTokenType {
    /// Returns true if this syntax kind is a Delphi keyword
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::Program
                | Self::Unit
                | Self::Interface
                | Self::Implementation
                | Self::Uses
                | Self::Type
                | Self::Var
                | Self::Const
                | Self::Function
                | Self::Procedure
                | Self::Begin
                | Self::End
                | Self::If
                | Self::Then
                | Self::Else
                | Self::While
                | Self::Do
                | Self::For
                | Self::To
                | Self::Downto
                | Self::Repeat
                | Self::Until
                | Self::Case
                | Self::Of
                | Self::With
                | Self::Try
                | Self::Except
                | Self::Finally
                | Self::Raise
                | Self::Class
                | Self::Object
                | Self::Record
                | Self::Array
                | Self::Set
                | Self::File
                | Self::Packed
                | Self::String_
                | Self::Integer
                | Self::Real
                | Self::Boolean
                | Self::Char
                | Self::Pointer
                | Self::Nil
                | Self::True_
                | Self::False_
                | Self::And_
                | Self::Or_
                | Self::Not_
                | Self::Div
                | Self::Mod
                | Self::In_
                | Self::Is_
                | Self::As_
        )
    }
}

impl TokenType for DelphiTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment | Self::LineComment | Self::BlockComment)
    }

    fn role(&self) -> Self::Role {
        if self.is_keyword() {
            return UniversalTokenRole::Keyword;
        }
        match self {
            Self::Identifier => UniversalTokenRole::Name,
            Self::String | Self::Number | Self::Float | Self::True_ | Self::False_ | Self::Nil => UniversalTokenRole::Literal,
            Self::Plus | Self::Minus | Self::Star | Self::Slash | Self::Equal | Self::NotEqual | Self::Less | Self::Greater | Self::LessEqual | Self::GreaterEqual | Self::Assign | Self::Caret | Self::At => UniversalTokenRole::Operator,
            Self::Dot | Self::DotDot | Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::Semicolon | Self::Comma | Self::Colon => UniversalTokenRole::Punctuation,
            Self::Comment | Self::LineComment | Self::BlockComment => UniversalTokenRole::Comment,
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Error => UniversalTokenRole::Error,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DelphiTokenType {
    Root,
    // Basic tokens
    Identifier,
    String,
    Number,
    Float,
    Whitespace,
    Newline,

    // Delphi keywords
    Program,
    Unit,
    Interface,
    Implementation,
    Uses,
    Type,
    Var,
    Const,
    Function,
    Procedure,
    Begin,
    End,
    If,
    Then,
    Else,
    While,
    Do,
    For,
    To,
    Downto,
    Repeat,
    Until,
    Case,
    Of,
    With,
    Try,
    Except,
    Finally,
    Raise,
    Class,
    Object,
    Record,
    Array,
    Set,
    File,
    Packed,
    String_,
    Integer,
    Real,
    Boolean,
    Char,
    Pointer,
    Nil,
    True_,
    False_,
    And_,
    Or_,
    Not_,
    Div,
    Mod,
    In_,
    Is_,
    As_,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Assign,
    Dot,
    DotDot,
    Caret,
    At,

    // Separators
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Semicolon,
    Comma,
    Colon,

    // Comments
    Comment,
    LineComment,
    BlockComment,

    // Special
    Error,
    Eof,
}
