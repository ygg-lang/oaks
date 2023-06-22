use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::Serialize;

/// Python 语法节点类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[repr(u16)]
pub enum PythonSyntaxKind {
    // 基础 kind
    Whitespace,
    Comment,
    Identifier,

    // 字面量
    Number,
    String,
    Bytes,
    FString,

    // 关键字
    AndKeyword,
    AsKeyword,
    AssertKeyword,
    AsyncKeyword,
    AwaitKeyword,
    BreakKeyword,
    ClassKeyword,
    ContinueKeyword,
    DefKeyword,
    DelKeyword,
    ElifKeyword,
    ElseKeyword,
    ExceptKeyword,
    FalseKeyword,
    FinallyKeyword,
    ForKeyword,
    FromKeyword,
    GlobalKeyword,
    IfKeyword,
    ImportKeyword,
    InKeyword,
    IsKeyword,
    LambdaKeyword,
    NoneKeyword,
    NonlocalKeyword,
    NotKeyword,
    OrKeyword,
    PassKeyword,
    RaiseKeyword,
    ReturnKeyword,
    TrueKeyword,
    TryKeyword,
    WhileKeyword,
    WithKeyword,
    YieldKeyword,

    // 运算符
    Plus,
    Minus,
    Star,
    DoubleStar,
    Slash,
    DoubleSlash,
    Percent,
    At,
    LeftShift,
    RightShift,
    Ampersand,
    Pipe,
    Caret,
    Tilde,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Equal,
    NotEqual,

    // 赋值运算符
    Assign,
    PlusAssign,
    MinusAssign,
    StarAssign,
    DoubleStarAssign,
    SlashAssign,
    DoubleSlashAssign,
    PercentAssign,
    AtAssign,
    AmpersandAssign,
    PipeAssign,
    CaretAssign,
    LeftShiftAssign,
    RightShiftAssign,

    // 分隔符
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Colon,
    Semicolon,
    Dot,
    Arrow,
    Ellipsis,

    // 特殊
    Newline,
    Indent,
    Dedent,
    Eof,
    Error,

    // 语法节点
    Root,
    Module,
    InteractiveModule,
    ExpressionModule,
    Suite,
    FunctionDef,
    AsyncFunctionDef,
    ClassDef,
    Return,
    Delete,
    AssignStmt,
    AugAssign,
    AnnAssign,
    For,
    AsyncFor,
    While,
    If,
    With,
    AsyncWith,
    Raise,
    Try,
    Assert,
    Import,
    ImportFrom,
    Global,
    Nonlocal,
    Expr,
    Pass,
    Break,
    Continue,

    // 表达式
    BoolOp,
    NamedExpr,
    BinOp,
    UnaryOp,
    Lambda,
    IfExp,
    Dict,
    Set,
    ListComp,
    SetComp,
    DictComp,
    GeneratorExp,
    Await,
    Yield,
    YieldFrom,
    Compare,
    Call,
    FormattedValue,
    JoinedStr,
    Constant,
    Attribute,
    Subscript,
    Starred,
    Name,
    List,
    Tuple,
    Slice,

    // 模式匹配 (Python 3.10+)
    Match,
    MatchValue,
    MatchSingleton,
    MatchSequence,
    MatchMapping,
    MatchClass,
    MatchStar,
    MatchAs,
    MatchOr,

    // 类型注解
    TypeIgnore,

    // 参数
    Arguments,
    Arg,
    Keyword,

    // 异常处理
    ExceptHandler,

    // 别名
    Alias,

    // With 项
    WithItem,

    // 推导式
    Comprehension,

    // 操作符
    Add,
    Sub,
    Mult,
    MatMult,
    Div,
    Mod,
    Pow,
    LShift,
    RShift,
    BitOr,
    BitXor,
    BitAnd,
    FloorDiv,

    // 一元操作符
    Invert,
    Not,
    UAdd,
    USub,

    // 比较操作符
    Eq,
    NotEq,
    Lt,
    LtE,
    Gt,
    GtE,
    Is,
    IsNot,
    In,
    NotIn,

    // 布尔操作符
    And,
    Or,

    // 表达式上下文
    Load,
    Store,
    Del,
}

impl From<u16> for PythonSyntaxKind {
    fn from(d: u16) -> PythonSyntaxKind {
        assert!(d <= (PythonSyntaxKind::Del as u16));
        unsafe { core::mem::transmute::<u16, PythonSyntaxKind>(d) }
    }
}

impl From<PythonSyntaxKind> for u16 {
    fn from(k: PythonSyntaxKind) -> u16 {
        k as u16
    }
}

impl PythonSyntaxKind {
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::AndKeyword
                | Self::AsKeyword
                | Self::AssertKeyword
                | Self::AsyncKeyword
                | Self::AwaitKeyword
                | Self::BreakKeyword
                | Self::ClassKeyword
                | Self::ContinueKeyword
                | Self::DefKeyword
                | Self::DelKeyword
                | Self::ElifKeyword
                | Self::ElseKeyword
                | Self::ExceptKeyword
                | Self::FalseKeyword
                | Self::FinallyKeyword
                | Self::ForKeyword
                | Self::FromKeyword
                | Self::GlobalKeyword
                | Self::IfKeyword
                | Self::ImportKeyword
                | Self::InKeyword
                | Self::IsKeyword
                | Self::LambdaKeyword
                | Self::NoneKeyword
                | Self::NonlocalKeyword
                | Self::NotKeyword
                | Self::OrKeyword
                | Self::PassKeyword
                | Self::RaiseKeyword
                | Self::ReturnKeyword
                | Self::TrueKeyword
                | Self::TryKeyword
                | Self::WhileKeyword
                | Self::WithKeyword
                | Self::YieldKeyword
        )
    }
}

impl TokenType for PythonSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline | Self::Indent | Self::Dedent => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Identifier => UniversalTokenRole::Name,
            Self::Number | Self::String | Self::Bytes | Self::FString => UniversalTokenRole::Literal,
            _ if self.is_keyword() => UniversalTokenRole::Keyword,
            Self::Plus
            | Self::Minus
            | Self::Star
            | Self::DoubleStar
            | Self::Slash
            | Self::DoubleSlash
            | Self::Percent
            | Self::At
            | Self::LeftShift
            | Self::RightShift
            | Self::Ampersand
            | Self::Pipe
            | Self::Caret
            | Self::Tilde
            | Self::Less
            | Self::Greater
            | Self::LessEqual
            | Self::GreaterEqual
            | Self::Equal
            | Self::NotEqual
            | Self::Assign
            | Self::PlusAssign
            | Self::MinusAssign
            | Self::StarAssign
            | Self::DoubleStarAssign
            | Self::SlashAssign
            | Self::DoubleSlashAssign
            | Self::PercentAssign
            | Self::AtAssign
            | Self::AmpersandAssign
            | Self::PipeAssign
            | Self::CaretAssign
            | Self::LeftShiftAssign
            | Self::RightShiftAssign => UniversalTokenRole::Operator,
            Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::LeftBrace | Self::RightBrace | Self::Comma | Self::Colon | Self::Semicolon | Self::Dot | Self::Arrow | Self::Ellipsis => UniversalTokenRole::Punctuation,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Indent | Self::Dedent)
    }
}

impl ElementType for PythonSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root | Self::Module => UniversalElementRole::Root,
            Self::FunctionDef
            | Self::AsyncFunctionDef
            | Self::ClassDef
            | Self::Return
            | Self::Delete
            | Self::AssignStmt
            | Self::AugAssign
            | Self::AnnAssign
            | Self::For
            | Self::AsyncFor
            | Self::While
            | Self::If
            | Self::With
            | Self::AsyncWith
            | Self::Raise
            | Self::Try
            | Self::Assert
            | Self::Import
            | Self::ImportFrom
            | Self::Global
            | Self::Nonlocal
            | Self::Expr
            | Self::Pass
            | Self::Break
            | Self::Continue => UniversalElementRole::Statement,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }
}
