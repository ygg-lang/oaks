use oak_core::{ElementType, Token, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

pub type PowerShellToken = Token<PowerShellSyntaxKind>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum PowerShellSyntaxKind {
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

impl TokenType for PowerShellSyntaxKind {
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

impl ElementType for PowerShellSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalElementRole::Error,
            Self::Root => UniversalElementRole::Root,
            Self::FunctionDef | Self::ClassDef | Self::IfStatement => UniversalElementRole::Detail,
            _ => UniversalElementRole::None,
        }
    }
}
