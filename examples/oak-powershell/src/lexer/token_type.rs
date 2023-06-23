use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type PowerShellToken = Token<PowerShellTokenType>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum PowerShellTokenType {
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

impl TokenType for PowerShellTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            _ => UniversalTokenRole::None,
        }
    }
}
