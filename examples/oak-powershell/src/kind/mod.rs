use oak_core::{SyntaxKind, Token};
use serde::{Deserialize, Serialize};

pub type PowerShellToken = Token<PowerShellSyntaxKind>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    Error,
    Eof,
}

impl SyntaxKind for PowerShellSyntaxKind {
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
        !matches!(self, Self::Root)
    }

    fn is_element_type(&self) -> bool {
        matches!(self, Self::Root)
    }
}
