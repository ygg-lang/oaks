#![doc = include_str!("../../readme.md")]

use oak_core::{ElementType, Token, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

pub type ValaToken = Token<ValaSyntaxKind>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ValaSyntaxKind {
    // 基础文本
    Text,
    Whitespace,
    Newline,

    // 错误处理
    Error,

    // EOF
    Eof,

    // 标点符号
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Colon,
    Semicolon,
    Dot,
    Comma,
    Question,
    Bang,
    At,
    Hash,
    Dollar,
    Percent,
    Caret,
    Ampersand,
    Star,
    Plus,
    Minus,
    Eq,
    LessThan,
    GreaterThan,
    Slash,
    Backslash,
    Pipe,
    Tilde,

    // 复合操作符
    EqEq,
    NotEq,
    LessEq,
    GreaterEq,
    AndAnd,
    OrOr,
    PlusPlus,
    MinusMinus,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PercentEq,
    LeftShift,
    RightShift,
    Arrow,

    // Vala 关键字
    AbstractKw,
    AsKw,
    BaseKw,
    BreakKw,
    CaseKw,
    CatchKw,
    ClassKw,
    ConstKw,
    ConstructKw,
    ContinueKw,
    DefaultKw,
    DelegateKw,
    DeleteKw,
    DoKw,
    ElseKw,
    EnumKw,
    EnsuresKw,
    ErrordomainKw,
    ExternKw,
    FalseKw,
    FinallyKw,
    ForKw,
    ForeachKw,
    GetKw,
    IfKw,
    InKw,
    InlineKw,
    InterfaceKw,
    InternalKw,
    IsKw,
    LockKw,
    NamespaceKw,
    NewKw,
    NullKw,
    OutKw,
    OverrideKw,
    OwnedKw,
    PrivateKw,
    ProtectedKw,
    PublicKw,
    RefKw,
    RequiresKw,
    ReturnKw,
    SetKw,
    SizeofKw,
    StaticKw,
    StructKw,
    SwitchKw,
    ThisKw,
    ThrowKw,
    ThrowsKw,
    TrueKw,
    TryKw,
    TypeofKw,
    UnownedKw,
    UsingKw,
    VarKw,
    VirtualKw,
    VoidKw,
    VolatileKw,
    WeakKw,
    WhileKw,
    YieldKw,

    // 基本类型
    BoolKw,
    CharKw,
    UcharKw,
    IntKw,
    UintKw,
    ShortKw,
    UshortKw,
    LongKw,
    UlongKw,
    Int8Kw,
    Uint8Kw,
    Int16Kw,
    Uint16Kw,
    Int32Kw,
    Uint32Kw,
    Int64Kw,
    Uint64Kw,
    FloatKw,
    DoubleKw,
    StringKw,

    // 字面量
    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
    CharLiteral,
    BoolLiteral,
    NullLiteral,
    Identifier,

    // 注释
    LineComment,
    SourceFile,
    BlockComment,
}

impl TokenType for ValaSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::LineComment | Self::BlockComment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }
}

impl ElementType for ValaSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::SourceFile => UniversalElementRole::Root,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}
