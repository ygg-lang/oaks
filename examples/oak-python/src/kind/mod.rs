use oak_core::SyntaxKind;

/// Python 语法节点类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl SyntaxKind for PythonSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment | Self::Newline)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_token_type(&self) -> bool {
        !matches!(
            self,
            Self::Root
                | Self::Module
                | Self::InteractiveModule
                | Self::ExpressionModule
                | Self::FunctionDef
                | Self::AsyncFunctionDef
                | Self::ClassDef
        )
    }

    fn is_element_type(&self) -> bool {
        matches!(
            self,
            Self::Root
                | Self::Module
                | Self::InteractiveModule
                | Self::ExpressionModule
                | Self::FunctionDef
                | Self::AsyncFunctionDef
                | Self::ClassDef
        )
    }
}
