#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SoliditySyntaxKind {
    // 空白和换
    Whitespace,
    Newline,

    // 注释
    LineComment,
    BlockComment,

    // 关键
    Contract,
    Interface,
    Library,
    Function,
    Modifier,
    Event,
    Struct,
    Enum,
    Mapping,
    Array,

    // 可见性修饰符
    Public,
    Private,
    Internal,
    External,

    // 状态修饰符
    Pure,
    View,
    Payable,
    Constant,

    // 类型关键
    Bool,
    String,
    Bytes,
    Address,
    Uint,
    Int,
    Fixed,
    Ufixed,

    // 控制
    If,
    Else,
    For,
    While,
    Do,
    Break,
    Continue,
    Return,
    Try,
    Catch,

    // 其他关键
    Import,
    Pragma,
    Using,
    Is,
    Override,
    Virtual,
    Abstract,

    // 字面
    NumberLiteral,
    StringLiteral,
    BooleanLiteral,
    AddressLiteral,
    HexLiteral,

    // 标识
    Identifier,

    // 操作
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Power,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
    Not,
    BitAnd,
    BitOr,
    BitXor,
    BitNot,
    LeftShift,
    RightShift,
    Assign,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
    PercentAssign,

    // 分隔
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Semicolon,
    Comma,
    Dot,
    Arrow,

    // 特殊
    Error,
    Eof,
}

impl oak_core::SyntaxKind for SoliditySyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::LineComment | Self::BlockComment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::LineComment | Self::BlockComment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_token_type(&self) -> bool {
        !matches!(self, Self::Error | Self::Eof)
    }

    fn is_element_type(&self) -> bool {
        false
    }
}
