use oak_core::{ElementType, UniversalElementRole};

/// Element types for Delphi.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DelphiElementType {
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

impl DelphiElementType {
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

impl ElementType for DelphiElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::DelphiTokenType> for DelphiElementType {
    fn from(token: crate::lexer::token_type::DelphiTokenType) -> Self {
        match token {
            crate::lexer::token_type::DelphiTokenType::Root => DelphiElementType::Root,
            crate::lexer::token_type::DelphiTokenType::Identifier => DelphiElementType::Identifier,
            crate::lexer::token_type::DelphiTokenType::String => DelphiElementType::String,
            crate::lexer::token_type::DelphiTokenType::Number => DelphiElementType::Number,
            crate::lexer::token_type::DelphiTokenType::Float => DelphiElementType::Float,
            crate::lexer::token_type::DelphiTokenType::Whitespace => DelphiElementType::Whitespace,
            crate::lexer::token_type::DelphiTokenType::Newline => DelphiElementType::Newline,
            crate::lexer::token_type::DelphiTokenType::Program => DelphiElementType::Program,
            crate::lexer::token_type::DelphiTokenType::Unit => DelphiElementType::Unit,
            crate::lexer::token_type::DelphiTokenType::Interface => DelphiElementType::Interface,
            crate::lexer::token_type::DelphiTokenType::Implementation => DelphiElementType::Implementation,
            crate::lexer::token_type::DelphiTokenType::Uses => DelphiElementType::Uses,
            crate::lexer::token_type::DelphiTokenType::Type => DelphiElementType::Type,
            crate::lexer::token_type::DelphiTokenType::Var => DelphiElementType::Var,
            crate::lexer::token_type::DelphiTokenType::Const => DelphiElementType::Const,
            crate::lexer::token_type::DelphiTokenType::Function => DelphiElementType::Function,
            crate::lexer::token_type::DelphiTokenType::Procedure => DelphiElementType::Procedure,
            crate::lexer::token_type::DelphiTokenType::Begin => DelphiElementType::Begin,
            crate::lexer::token_type::DelphiTokenType::End => DelphiElementType::End,
            crate::lexer::token_type::DelphiTokenType::If => DelphiElementType::If,
            crate::lexer::token_type::DelphiTokenType::Then => DelphiElementType::Then,
            crate::lexer::token_type::DelphiTokenType::Else => DelphiElementType::Else,
            crate::lexer::token_type::DelphiTokenType::While => DelphiElementType::While,
            crate::lexer::token_type::DelphiTokenType::Do => DelphiElementType::Do,
            crate::lexer::token_type::DelphiTokenType::For => DelphiElementType::For,
            crate::lexer::token_type::DelphiTokenType::To => DelphiElementType::To,
            crate::lexer::token_type::DelphiTokenType::Downto => DelphiElementType::Downto,
            crate::lexer::token_type::DelphiTokenType::Repeat => DelphiElementType::Repeat,
            crate::lexer::token_type::DelphiTokenType::Until => DelphiElementType::Until,
            crate::lexer::token_type::DelphiTokenType::Case => DelphiElementType::Case,
            crate::lexer::token_type::DelphiTokenType::Of => DelphiElementType::Of,
            crate::lexer::token_type::DelphiTokenType::With => DelphiElementType::With,
            crate::lexer::token_type::DelphiTokenType::Try => DelphiElementType::Try,
            crate::lexer::token_type::DelphiTokenType::Except => DelphiElementType::Except,
            crate::lexer::token_type::DelphiTokenType::Finally => DelphiElementType::Finally,
            crate::lexer::token_type::DelphiTokenType::Raise => DelphiElementType::Raise,
            crate::lexer::token_type::DelphiTokenType::Class => DelphiElementType::Class,
            crate::lexer::token_type::DelphiTokenType::Object => DelphiElementType::Object,
            crate::lexer::token_type::DelphiTokenType::Record => DelphiElementType::Record,
            crate::lexer::token_type::DelphiTokenType::Array => DelphiElementType::Array,
            crate::lexer::token_type::DelphiTokenType::Set => DelphiElementType::Set,
            crate::lexer::token_type::DelphiTokenType::File => DelphiElementType::File,
            crate::lexer::token_type::DelphiTokenType::Packed => DelphiElementType::Packed,
            crate::lexer::token_type::DelphiTokenType::String_ => DelphiElementType::String_,
            crate::lexer::token_type::DelphiTokenType::Integer => DelphiElementType::Integer,
            crate::lexer::token_type::DelphiTokenType::Real => DelphiElementType::Real,
            crate::lexer::token_type::DelphiTokenType::Boolean => DelphiElementType::Boolean,
            crate::lexer::token_type::DelphiTokenType::Char => DelphiElementType::Char,
            crate::lexer::token_type::DelphiTokenType::Pointer => DelphiElementType::Pointer,
            crate::lexer::token_type::DelphiTokenType::Nil => DelphiElementType::Nil,
            crate::lexer::token_type::DelphiTokenType::True_ => DelphiElementType::True_,
            crate::lexer::token_type::DelphiTokenType::False_ => DelphiElementType::False_,
            crate::lexer::token_type::DelphiTokenType::And_ => DelphiElementType::And_,
            crate::lexer::token_type::DelphiTokenType::Or_ => DelphiElementType::Or_,
            crate::lexer::token_type::DelphiTokenType::Not_ => DelphiElementType::Not_,
            crate::lexer::token_type::DelphiTokenType::Div => DelphiElementType::Div,
            crate::lexer::token_type::DelphiTokenType::Mod => DelphiElementType::Mod,
            crate::lexer::token_type::DelphiTokenType::In_ => DelphiElementType::In_,
            crate::lexer::token_type::DelphiTokenType::Is_ => DelphiElementType::Is_,
            crate::lexer::token_type::DelphiTokenType::As_ => DelphiElementType::As_,
            crate::lexer::token_type::DelphiTokenType::Plus => DelphiElementType::Plus,
            crate::lexer::token_type::DelphiTokenType::Minus => DelphiElementType::Minus,
            crate::lexer::token_type::DelphiTokenType::Star => DelphiElementType::Star,
            crate::lexer::token_type::DelphiTokenType::Slash => DelphiElementType::Slash,
            crate::lexer::token_type::DelphiTokenType::Equal => DelphiElementType::Equal,
            crate::lexer::token_type::DelphiTokenType::NotEqual => DelphiElementType::NotEqual,
            crate::lexer::token_type::DelphiTokenType::Less => DelphiElementType::Less,
            crate::lexer::token_type::DelphiTokenType::Greater => DelphiElementType::Greater,
            crate::lexer::token_type::DelphiTokenType::LessEqual => DelphiElementType::LessEqual,
            crate::lexer::token_type::DelphiTokenType::GreaterEqual => DelphiElementType::GreaterEqual,
            crate::lexer::token_type::DelphiTokenType::Assign => DelphiElementType::Assign,
            crate::lexer::token_type::DelphiTokenType::Dot => DelphiElementType::Dot,
            crate::lexer::token_type::DelphiTokenType::DotDot => DelphiElementType::DotDot,
            crate::lexer::token_type::DelphiTokenType::Caret => DelphiElementType::Caret,
            crate::lexer::token_type::DelphiTokenType::At => DelphiElementType::At,
            crate::lexer::token_type::DelphiTokenType::LeftParen => DelphiElementType::LeftParen,
            crate::lexer::token_type::DelphiTokenType::RightParen => DelphiElementType::RightParen,
            crate::lexer::token_type::DelphiTokenType::LeftBracket => DelphiElementType::LeftBracket,
            crate::lexer::token_type::DelphiTokenType::RightBracket => DelphiElementType::RightBracket,
            crate::lexer::token_type::DelphiTokenType::Semicolon => DelphiElementType::Semicolon,
            crate::lexer::token_type::DelphiTokenType::Comma => DelphiElementType::Comma,
            crate::lexer::token_type::DelphiTokenType::Colon => DelphiElementType::Colon,
            crate::lexer::token_type::DelphiTokenType::Comment => DelphiElementType::Comment,
            crate::lexer::token_type::DelphiTokenType::LineComment => DelphiElementType::LineComment,
            crate::lexer::token_type::DelphiTokenType::BlockComment => DelphiElementType::BlockComment,
            crate::lexer::token_type::DelphiTokenType::Error => DelphiElementType::Error,
            crate::lexer::token_type::DelphiTokenType::Eof => DelphiElementType::Eof,
        }
    }
}
