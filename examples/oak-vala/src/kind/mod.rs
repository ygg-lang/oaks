#![doc = include_str!("../../readme.md")]

use oak_core::{SyntaxKind, Token};
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
    BlockComment,
}

impl SyntaxKind for ValaSyntaxKind {
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
        true // Vala 语言的所有语法类型都是 token 类型
    }

    fn is_element_type(&self) -> bool {
        false // Vala 语言没有元素类型
    }
}
