use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

impl TokenType for PhpSyntaxKind {
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

impl PhpSyntaxKind {
    pub fn is_token(&self) -> bool {
        !self.is_element()
    }
}

impl ElementType for PhpSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalElementRole::Error,
            Self::Root => UniversalElementRole::Root,
            Self::ClassDef | Self::FunctionDef | Self::NamespaceDef => UniversalElementRole::Detail,
            Self::Literal | Self::ParenthesizedExpression | Self::CallExpression | Self::ArrayAccessExpression | Self::MemberAccessExpression | Self::BinaryExpression => UniversalElementRole::Expression,
            _ => UniversalElementRole::None,
        }
    }
}

impl PhpSyntaxKind {
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
