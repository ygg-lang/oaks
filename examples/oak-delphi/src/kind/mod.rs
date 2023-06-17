use oak_core::SyntaxKind;

/// Delphi 语法种类
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DelphiSyntaxKind {
    // 基本 token
    Identifier,
    String,
    Number,
    Whitespace,
    Newline,

    // Delphi 关键字
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

    // 操作符
    Plus,         // +
    Minus,        // -
    Star,         // *
    Slash,        // /
    Equal,        // =
    NotEqual,     // <>
    Less,         // <
    Greater,      // >
    LessEqual,    // <=
    GreaterEqual, // >=
    Assign,       // :=
    Dot,          // .
    DotDot,       // ..
    Caret,        // ^
    At,           // @

    // 分隔符
    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    Semicolon,    // ;
    Comma,        // ,
    Colon,        // :

    // 注释
    Comment,

    // 特殊
    Error,
    Eof,
}

impl DelphiSyntaxKind {
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

impl SyntaxKind for DelphiSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_token_type(&self) -> bool {
        !matches!(self, Self::Error | Self::Eof)
    }

    fn is_element_type(&self) -> bool {
        false
    }
}
