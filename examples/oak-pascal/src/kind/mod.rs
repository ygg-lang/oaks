use oak_core::{ElementType, Token, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

pub type PascalToken = Token<PascalSyntaxKind>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PascalSyntaxKind {
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

    // Element kinds
    Root,
    ProgramBlock,
    VarSection,
    ConstSection,
    TypeSection,
    ProcedureDef,
    FunctionDef,
    CompoundStmt,
    IfStmt,
    WhileStmt,
    ForStmt,
    Expression,

    // 特殊
    Error,
    Eof,
}

impl TokenType for PascalSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }
}

impl ElementType for PascalSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalElementRole::Error,
            Self::Root => UniversalElementRole::Root,
            Self::ProgramBlock | Self::ProcedureDef | Self::FunctionDef | Self::VarSection => UniversalElementRole::Detail,
            _ => UniversalElementRole::None,
        }
    }
}
