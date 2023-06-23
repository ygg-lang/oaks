use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type PhpToken = Token<PhpTokenType>;

impl PhpTokenType {
    /// Checks if this syntax kind represents a token (leaf node).
    pub fn is_token(&self) -> bool {
        !self.is_element()
    }
}

impl PhpTokenType {
    /// Checks if this syntax kind represents a composite element (non-leaf node).
    pub fn is_element(&self) -> bool {
        matches!(
            self,
            Self::Root
                | Self::ClassDef
                | Self::FunctionDef
                | Self::MethodDef
                | Self::PropertyDef
                | Self::ConstDef
                | Self::TraitDef
                | Self::InterfaceDef
                | Self::NamespaceDef
                | Self::UseStatement
                | Self::IfStatement
                | Self::WhileStatement
                | Self::DoWhileStatement
                | Self::ForStatement
                | Self::ForeachStatement
                | Self::SwitchStatement
                | Self::TryStatement
                | Self::CatchBlock
                | Self::FinallyBlock
                | Self::ExpressionStatement
                | Self::ReturnStatement
                | Self::ThrowStatement
                | Self::BreakStatement
                | Self::ContinueStatement
                | Self::EchoStatement
                | Self::GlobalStatement
                | Self::StaticStatement
                | Self::UnsetStatement
                | Self::CompoundStatement
                | Self::Literal
                | Self::ParenthesizedExpression
                | Self::CallExpression
                | Self::ArrayAccessExpression
                | Self::MemberAccessExpression
                | Self::BinaryExpression
        )
    }
}

impl TokenType for PhpTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Error;

    fn is_ignored(&self) -> bool {
        false
    }

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalTokenRole::None,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PhpTokenType {
    // Whitespace and newlines
    Whitespace,
    Newline,

    // Comments
    Comment,

    // Literals
    StringLiteral,
    NumberLiteral,
    BooleanLiteral,
    NullLiteral,

    // Identifiers and keywords
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

    // Operators
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

    // Punctuations
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

    // PHP special tags
    OpenTag,
    CloseTag,
    EchoTag,

    // Special
    Eof,
    Error,

    // Element types
    Root,
    ClassDef,
    FunctionDef,
    MethodDef,
    PropertyDef,
    ConstDef,
    TraitDef,
    InterfaceDef,
    NamespaceDef,
    UseStatement,
    IfStatement,
    WhileStatement,
    DoWhileStatement,
    ForStatement,
    ForeachStatement,
    SwitchStatement,
    TryStatement,
    CatchBlock,
    FinallyBlock,
    ExpressionStatement,
    ReturnStatement,
    ThrowStatement,
    BreakStatement,
    ContinueStatement,
    EchoStatement,
    GlobalStatement,
    StaticStatement,
    UnsetStatement,
    CompoundStatement,

    // Expressions
    Literal,
    ParenthesizedExpression,
    CallExpression,
    ArrayAccessExpression,
    MemberAccessExpression,
    BinaryExpression,
}
