use oak_core::SyntaxKind;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PhpSyntaxKind {
    // 空白字符和换行
    Whitespace,
    Newline,

    // 注释
    Comment,

    // 字面量
    StringLiteral,
    NumberLiteral,
    BooleanLiteral,
    NullLiteral,

    // 标识符和关键字
    Identifier,
    Variable,
    Abstract,
    And,
    Array,
    As,
    Break,
    Callable,
    Case,
    Catch,
    Class,
    Clone,
    Const,
    Continue,
    Declare,
    Default,
    Do,
    Echo,
    Else,
    Elseif,
    Empty,
    Enddeclare,
    Endfor,
    Endforeach,
    Endif,
    Endswitch,
    Endwhile,
    Eval,
    Exit,
    Extends,
    Final,
    Finally,
    For,
    Foreach,
    Function,
    Global,
    Goto,
    If,
    Implements,
    Include,
    IncludeOnce,
    Instanceof,
    Insteadof,
    Interface,
    Isset,
    List,
    Namespace,
    New,
    Or,
    Print,
    Private,
    Protected,
    Public,
    Require,
    RequireOnce,
    Return,
    Static,
    Switch,
    Throw,
    Trait,
    Try,
    Unset,
    Use,
    Var,
    While,
    Xor,
    Yield,
    YieldFrom,

    // 运算符
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Power,
    Concat,
    Equal,
    Identical,
    NotEqual,
    NotIdentical,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Spaceship,
    LogicalAnd,
    LogicalOr,
    LogicalXor,
    LogicalNot,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,
    LeftShift,
    RightShift,
    Assign,
    PlusAssign,
    MinusAssign,
    MultiplyAssign,
    DivideAssign,
    ModuloAssign,
    PowerAssign,
    ConcatAssign,
    BitwiseAndAssign,
    BitwiseOrAssign,
    BitwiseXorAssign,
    LeftShiftAssign,
    RightShiftAssign,
    Increment,
    Decrement,
    Arrow,
    DoubleArrow,
    NullCoalesce,
    NullCoalesceAssign,
    Ellipsis,

    // 分隔符
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Semicolon,
    Comma,
    Dot,
    Question,
    Colon,
    DoubleColon,
    Backslash,
    At,
    Dollar,

    // PHP 特殊标记
    OpenTag,
    CloseTag,
    EchoTag,

    // 特殊
    Eof,
    Error,
}

impl SyntaxKind for PhpSyntaxKind {
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
        true // PHP doesn't have element types in this simple implementation
    }

    fn is_element_type(&self) -> bool {
        false // PHP doesn't have element types in this simple implementation
    }
}

pub type PhpNode = oak_core::tree::RedNode<PhpSyntaxKind>;
