use oak_core::SyntaxKind;

/// Crystal 语法节点类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum CrystalSyntaxKind {
    // 基础 kind
    Whitespace,
    Comment,
    Identifier,

    // 字面量
    Number,
    String,
    Character,
    Symbol,

    // 关键字
    ClassKeyword,
    ModuleKeyword,
    DefKeyword,
    EndKeyword,
    IfKeyword,
    ElseKeyword,
    ElsifKeyword,
    UnlessKeyword,
    CaseKeyword,
    WhenKeyword,
    ThenKeyword,
    WhileKeyword,
    UntilKeyword,
    ForKeyword,
    InKeyword,
    DoKeyword,
    BeginKeyword,
    RescueKeyword,
    EnsureKeyword,
    BreakKeyword,
    NextKeyword,
    ReturnKeyword,
    YieldKeyword,
    SuperKeyword,
    SelfKeyword,
    TrueKeyword,
    FalseKeyword,
    NilKeyword,
    AndKeyword,
    OrKeyword,
    NotKeyword,

    // 运算符
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    StarStar,
    Equal,
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Spaceship,
    Match,
    NotMatch,
    And,
    Or,
    Not,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,
    LeftShift,
    RightShift,
    LogicalAnd,
    LogicalOr,

    // 赋值运算符
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    PercentEqual,
    StarStarEqual,
    AndEqual,
    OrEqual,
    XorEqual,
    LeftShiftEqual,
    RightShiftEqual,
    LogicalAndEqual,
    LogicalOrEqual,

    // 分隔符
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Semicolon,
    Dot,
    DotDot,
    DotDotDot,
    Colon,
    DoubleColon,
    Arrow,
    FatArrow,
    Question,
    At,
    DoubleAt,
    Dollar,

    // 特殊
    Newline,
    Eof,
    Error,

    // 语法节点
    Root,
    Program,
    ClassDef,
    ModuleDef,
    MethodDef,
    Block,
    IfExpr,
    UnlessExpr,
    CaseExpr,
    WhenClause,
    WhileExpr,
    UntilExpr,
    ForExpr,
    BeginExpr,
    RescueClause,
    EnsureClause,
    CallExpr,
    IndexExpr,
    MemberExpr,
    BinaryExpr,
    UnaryExpr,
    AssignExpr,
    LiteralExpr,
    IdentifierExpr,
    ArrayExpr,
    HashExpr,
    HashPair,
    BlockExpr,
    LambdaExpr,
    YieldExpr,
    ReturnExpr,
    BreakExpr,
    NextExpr,
    SuperExpr,
    SelfExpr,
    ParenExpr,

    // 类型相关
    TypeExpr,
    GenericType,
    UnionType,
    TupleType,
    NamedTupleType,
    ProcType,

    // 模式匹配
    Pattern,
    IdentifierPattern,
    LiteralPattern,
    ArrayPattern,
    HashPattern,
    TuplePattern,

    // 参数
    ParamList,
    Param,
    SplatParam,
    DoubleSplatParam,
    BlockParam,

    // 注解
    Annotation,

    // 宏
    MacroDef,
    MacroCall,
    MacroExpr,

    // 其他
    Alias,
    Include,
    Extend,
    Require,

    // 可见性修饰符
    Private,
    Protected,
    Public,

    // 抽象和虚拟
    Abstract,
    Virtual,
    Override,

    // 结构体和枚举
    StructDef,
    EnumDef,
    UnionDef,
    LibDef,

    // 异常处理
    RaiseExpr,

    // 范围
    RangeExpr,
    ExclusiveRangeExpr,

    // 正则表达式
    RegexLiteral,

    // 字符串插值
    StringInterpolation,
    InterpolationExpr,

    // 符号
    SymbolLiteral,

    // 常量
    ConstantRef,

    // 实例变量和类变量
    InstanceVar,
    ClassVar,

    // 全局变量
    GlobalVar,

    // 特殊方法
    Getter,
    Setter,

    // 操作符重载
    OperatorDef,
}

impl CrystalSyntaxKind {
    pub fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment | Self::Newline)
    }

    pub fn is_keyword(self) -> bool {
        matches!(
            self,
            Self::ClassKeyword
                | Self::ModuleKeyword
                | Self::DefKeyword
                | Self::EndKeyword
                | Self::IfKeyword
                | Self::ElseKeyword
                | Self::ElsifKeyword
                | Self::UnlessKeyword
                | Self::CaseKeyword
                | Self::WhenKeyword
                | Self::ThenKeyword
                | Self::WhileKeyword
                | Self::UntilKeyword
                | Self::ForKeyword
                | Self::InKeyword
                | Self::DoKeyword
                | Self::BeginKeyword
                | Self::RescueKeyword
                | Self::EnsureKeyword
                | Self::BreakKeyword
                | Self::NextKeyword
                | Self::ReturnKeyword
                | Self::YieldKeyword
                | Self::SuperKeyword
                | Self::SelfKeyword
                | Self::TrueKeyword
                | Self::FalseKeyword
                | Self::NilKeyword
                | Self::AndKeyword
                | Self::OrKeyword
                | Self::NotKeyword
        )
    }

    pub fn is_literal(self) -> bool {
        matches!(self, Self::Number | Self::String | Self::Character | Self::Symbol | Self::RegexLiteral | Self::SymbolLiteral)
    }

    pub fn is_operator(self) -> bool {
        matches!(
            self,
            Self::Plus
                | Self::Minus
                | Self::Star
                | Self::Slash
                | Self::Percent
                | Self::StarStar
                | Self::Equal
                | Self::EqualEqual
                | Self::NotEqual
                | Self::Less
                | Self::LessEqual
                | Self::Greater
                | Self::GreaterEqual
                | Self::Spaceship
                | Self::Match
                | Self::NotMatch
                | Self::And
                | Self::Or
                | Self::Not
                | Self::BitwiseAnd
                | Self::BitwiseOr
                | Self::BitwiseXor
                | Self::BitwiseNot
                | Self::LeftShift
                | Self::RightShift
                | Self::LogicalAnd
                | Self::LogicalOr
        )
    }

    pub fn is_assignment_operator(self) -> bool {
        matches!(
            self,
            Self::PlusEqual
                | Self::MinusEqual
                | Self::StarEqual
                | Self::SlashEqual
                | Self::PercentEqual
                | Self::StarStarEqual
                | Self::AndEqual
                | Self::OrEqual
                | Self::XorEqual
                | Self::LeftShiftEqual
                | Self::RightShiftEqual
                | Self::LogicalAndEqual
                | Self::LogicalOrEqual
        )
    }

    pub fn is_delimiter(self) -> bool {
        matches!(
            self,
            Self::LeftParen
                | Self::RightParen
                | Self::LeftBrace
                | Self::RightBrace
                | Self::LeftBracket
                | Self::RightBracket
                | Self::Comma
                | Self::Semicolon
        )
    }
}

impl SyntaxKind for CrystalSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace)
    }

    fn is_token_type(&self) -> bool {
        !matches!(self, Self::Eof | Self::Error)
    }

    fn is_element_type(&self) -> bool {
        matches!(self, Self::Eof | Self::Error)
    }
}
