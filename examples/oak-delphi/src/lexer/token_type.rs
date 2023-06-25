use oak_core::{Token, TokenType, UniversalTokenRole};

pub type _DelphiToken = Token<DelphiTokenType>;

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

/// Represents the different types of tokens in the Delphi language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DelphiTokenType {
    /// Root node.
    Root,
    // Basic tokens
    /// Identifier.
    Identifier,
    /// String literal.
    String,
    /// Number literal.
    Number,
    /// Floating point literal.
    Float,
    /// Whitespace.
    Whitespace,
    /// Newline.
    Newline,

    // Delphi keywords
    /// `program` keyword.
    Program,
    /// `unit` keyword.
    Unit,
    /// `interface` keyword.
    Interface,
    /// `implementation` keyword.
    Implementation,
    /// `uses` keyword.
    Uses,
    /// `type` keyword.
    Type,
    /// `var` keyword.
    Var,
    /// `const` keyword.
    Const,
    /// `function` keyword.
    Function,
    /// `procedure` keyword.
    Procedure,
    /// `begin` keyword.
    Begin,
    /// `end` keyword.
    End,
    /// `if` keyword.
    If,
    /// `then` keyword.
    Then,
    /// `else` keyword.
    Else,
    /// `while` keyword.
    While,
    /// `do` keyword.
    Do,
    /// `for` keyword.
    For,
    /// `to` keyword.
    To,
    /// `downto` keyword.
    Downto,
    /// `repeat` keyword.
    Repeat,
    /// `until` keyword.
    Until,
    /// `case` keyword.
    Case,
    /// `of` keyword.
    Of,
    /// `with` keyword.
    With,
    /// `try` keyword.
    Try,
    /// `except` keyword.
    Except,
    /// `finally` keyword.
    Finally,
    /// `raise` keyword.
    Raise,
    /// `class` keyword.
    Class,
    /// `object` keyword.
    Object,
    /// `record` keyword.
    Record,
    /// `array` keyword.
    Array,
    /// `set` keyword.
    Set,
    /// `file` keyword.
    File,
    /// `packed` keyword.
    Packed,
    /// `string` keyword/type.
    String_,
    /// `integer` keyword/type.
    Integer,
    /// `real` keyword/type.
    Real,
    /// `boolean` keyword/type.
    Boolean,
    /// `char` keyword/type.
    Char,
    /// `pointer` keyword/type.
    Pointer,
    /// `nil` keyword.
    Nil,
    /// `true` literal.
    True_,
    /// `false` literal.
    False_,
    /// `and` keyword/operator.
    And_,
    /// `or` keyword/operator.
    Or_,
    /// `not` keyword/operator.
    Not_,
    /// `div` keyword/operator.
    Div,
    /// `mod` keyword/operator.
    Mod,
    /// `in` keyword/operator.
    In_,
    /// `is` keyword/operator.
    Is_,
    /// `as` keyword/operator.
    As_,

    // Operators
    /// Plus `+`.
    Plus,
    /// Minus `-`.
    Minus,
    /// Star `*`.
    Star,
    /// Slash `/`.
    Slash,
    /// Equal `=`.
    Equal,
    /// Not equal `<>`.
    NotEqual,
    /// Less than `<`.
    Less,
    /// Greater than `>`.
    Greater,
    /// Less than or equal `<=`.
    LessEqual,
    /// Greater than or equal `>=`.
    GreaterEqual,
    /// Assignment `:=`.
    Assign,
    /// Dot `.`.
    Dot,
    /// Range `..`.
    DotDot,
    /// Caret `^`.
    Caret,
    /// At symbol `@`.
    At,

    // Separators
    /// Left parenthesis `(`.
    LeftParen,
    /// Right parenthesis `)`.
    RightParen,
    /// Left bracket `[`.
    LeftBracket,
    /// Right bracket `]`.
    RightBracket,
    /// Semicolon `;`.
    Semicolon,
    /// Comma `,`.
    Comma,
    /// Colon `:`.
    Colon,

    // Comments
    /// Comment.
    Comment,
    /// Line comment `//`.
    LineComment,
    /// Block comment `{ ... }` or `(* ... *)`.
    BlockComment,

    // Special
    /// Error node.
    Error,
    /// End of file.
    Eof,
}
