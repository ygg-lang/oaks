use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PascalTokenType {
    // 空白和换行
    Whitespace,
    Newline,

    // 注释
    Comment,

    // 关键字
    Program,
    Begin,
    End,
    Var,
    Const,
    Type,
    Function,
    Procedure,
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
    Record,
    Array,
    Set,
    File,
    Packed,
    Nil,
    True,
    False,
    And,
    Or,
    Not,
    Div,
    Mod,
    In,

    // 标识符和字面量
    Identifier,
    IntegerLiteral,
    RealLiteral,
    StringLiteral,
    CharLiteral,

    // 运算符
    Plus,         // +
    Minus,        // -
    Multiply,     // *
    Divide,       // /
    Assign,       // :=
    Equal,        // =
    NotEqual,     // <>
    Less,         // <
    LessEqual,    // <=
    Greater,      // >
    GreaterEqual, // >=

    // 分隔符
    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    Semicolon,    // ;
    Comma,        // ,
    Dot,          // .
    Colon,        // :
    Range,        // ..
    Caret,        // ^

    // Element markers (Internal use)
    Root,
    ProgramBlock,
    VarSection,
    ConstSection,
    TypeSection,
    ProcedureDef,
    FunctionDef,
    CompoundStmt,
    Expression,

    // 其他
    Error,
    Eof,
}

pub type PascalToken = Token<PascalTokenType>;

impl TokenType for PascalTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Error;

    fn role(&self) -> Self::Role {
        use UniversalTokenRole::*;
        match self {
            Self::Whitespace | Self::Newline => Whitespace,
            Self::Comment => Comment,
            Self::Identifier => Name,
            Self::IntegerLiteral | Self::RealLiteral | Self::StringLiteral | Self::CharLiteral => Literal,
            Self::Program
            | Self::Begin
            | Self::End
            | Self::Var
            | Self::Const
            | Self::Type
            | Self::Function
            | Self::Procedure
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
            | Self::Record
            | Self::Array
            | Self::Set
            | Self::File
            | Self::Packed
            | Self::Nil
            | Self::True
            | Self::False
            | Self::And
            | Self::Or
            | Self::Not
            | Self::Div
            | Self::Mod
            | Self::In => Keyword,
            Self::Plus | Self::Minus | Self::Multiply | Self::Divide | Self::Assign | Self::Equal | Self::NotEqual | Self::Less | Self::LessEqual | Self::Greater | Self::GreaterEqual | Self::Caret => Operator,
            Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::Semicolon | Self::Comma | Self::Dot | Self::Colon | Self::Range => Punctuation,
            Self::Error => Error,
            Self::Eof => Eof,
            _ => None,
        }
    }
}
