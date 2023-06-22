use oak_core::{ElementType, Token, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

/// Token type for Delphi language syntax
pub type DelphiToken = Token<DelphiSyntaxKind>;

/// Syntax kinds for Delphi programming language
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DelphiSyntaxKind {
    /// Root node of the syntax tree
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

impl DelphiSyntaxKind {
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

impl TokenType for DelphiSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::Newline => UniversalTokenRole::Whitespace,
            Self::Identifier => UniversalTokenRole::Name,
            Self::String | Self::Number | Self::Float | Self::True_ | Self::False_ | Self::Nil => UniversalTokenRole::Literal,
            kind if kind.is_keyword() => UniversalTokenRole::Keyword,
            Self::Plus | Self::Minus | Self::Star | Self::Slash | Self::Equal | Self::NotEqual | Self::Less | Self::Greater | Self::LessEqual | Self::GreaterEqual | Self::Assign | Self::Dot | Self::DotDot | Self::Caret | Self::At => {
                UniversalTokenRole::Operator
            }
            Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::Semicolon | Self::Comma | Self::Colon => UniversalTokenRole::Punctuation,
            Self::Comment | Self::LineComment | Self::BlockComment => UniversalTokenRole::Comment,
            Self::Error => UniversalTokenRole::Error,
            Self::Eof => UniversalTokenRole::None,
            _ => UniversalTokenRole::None,
        }
    }
}

impl ElementType for DelphiSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            _ => UniversalElementRole::None,
        }
    }
}
