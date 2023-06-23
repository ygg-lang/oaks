use oak_core::{ElementType, Parser, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum PowerShellElementType {
    // Whitespace and comments
    Whitespace,
    Newline,
    Comment,

    // Keywords
    Begin,
    Break,
    Catch,
    Class,
    Continue,
    Data,
    Define,
    Do,
    DynamicParam,
    Else,
    ElseIf,
    End,
    Exit,
    Filter,
    Finally,
    For,
    ForEach,
    From,
    Function,
    If,
    In,
    Param,
    Process,
    Return,
    Switch,
    Throw,
    Trap,
    Try,
    Until,
    Using,
    Var,
    While,
    Workflow,

    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterEqual,
    LessEqual,
    Like,
    NotLike,
    Match,
    NotMatch,
    Contains,
    NotContains,
    NotIn,
    Replace,
    Split,
    Join,
    Is,
    IsNot,
    As,
    And,
    Or,
    Xor,
    Not,
    Band,
    Bor,
    Bxor,
    Bnot,
    Shl,
    Shr,

    // Punctuation
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Semicolon,
    Comma,
    Dot,
    DotDot,
    Colon,
    DoubleColon,
    Pipe,
    Ampersand,
    At,
    Dollar,
    Question,
    Exclamation,
    Backtick,
    SingleQuote,
    DoubleQuote,

    // Literals
    StringLiteral,
    NumberLiteral,
    BooleanLiteral,
    NullLiteral,
    ArrayLiteral,
    HashLiteral,

    // Identifiers and variables
    Identifier,
    Variable,
    AutomaticVariable,
    PreferenceVariable,

    // Special
    Root,
    FunctionDef,
    ClassDef,
    IfStatement,
    ForStatement,
    ForEachStatement,
    WhileStatement,
    DoWhileStatement,
    SwitchStatement,
    TryStatement,
    CatchBlock,
    FinallyBlock,
    ParamBlock,
    ProcessBlock,
    BeginBlock,
    EndBlock,
    ExpressionStatement,
    Pipeline,
    Command,
    CommandParameter,
    CommandArgument,
    Error,
    Eof,
}

impl ElementType for PowerShellElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::PowerShellTokenType> for PowerShellElementType {
    fn from(token: crate::lexer::token_type::PowerShellTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
