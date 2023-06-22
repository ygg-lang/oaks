#![doc = include_str!("../../readme.md")]

use oak_core::language::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VocSyntaxKind {
    // 基础文本
    Text,
    Whitespace,
    Newline,

    // 错误处理
    Error,

    // EOF
    Eof,

    // 根节点
    SourceFile,

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
    PlusEq,       // +=
    MinusEq,      // -=
    StarEq,       // *=
    SlashEq,      // /=
    PercentEq,    // %=
    AmpersandEq,  // &=
    PipeEq,       // |=
    CaretEq,      // ^=
    LeftShiftEq,  // <<=
    RightShiftEq, // >>=
    EqEq,         // ==
    Ne,           // !=
    Le,           // <=
    Ge,           // >=
    LeftShift,    // <<
    RightShift,   // >>
    AndAnd,       // &&
    OrOr,         // ||
    PlusPlus,     // ++
    MinusMinus,   // --
    Arrow,        // ->
    FatArrow,     // =>
    DotDot,       // ..
    DotDotDot,    // ...

    // V 语言关键字
    ModuleKw,
    ImportKw,
    PubKw,
    FnKw,
    StructKw,
    InterfaceKw,
    EnumKw,
    TypeKw,
    ConstKw,
    MutKw,
    SharedKw,
    VolatileKw,
    UnsafeKw,
    IfKw,
    ElseKw,
    ForKw,
    InKw,
    MatchKw,
    OrKw,
    ReturnKw,
    BreakKw,
    ContinueKw,
    GotoKw,
    DeferKw,
    GoKw,
    SelectKw,
    LockKw,
    RlockKw,
    AsKw,
    IsKw,
    SizeofKw,
    TypeofKw,
    OffsetofKw,
    AssertKw,
    PanicKw,
    EprintlnKw,
    PrintlnKw,
    PrintKw,
    EprintKw,

    // 基本类型
    BoolKw,
    I8Kw,
    I16Kw,
    I32Kw,
    I64Kw,
    U8Kw,
    U16Kw,
    U32Kw,
    U64Kw,
    IntKw,
    UintKw,
    F32Kw,
    F64Kw,
    StringKw,
    RuneKw,
    ByteKw,
    VoidptrKw,
    CharKw,

    // 字面量
    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
    CharLiteral,
    BoolLiteral,

    // 标识符
    Identifier,

    // 注释
    Comment,
}

impl TokenType for VocSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Error => UniversalTokenRole::Error,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Identifier => UniversalTokenRole::Name,
            Self::IntegerLiteral | Self::FloatLiteral | Self::StringLiteral | Self::CharLiteral | Self::BoolLiteral => UniversalTokenRole::Literal,
            Self::Plus
            | Self::Minus
            | Self::Star
            | Self::Slash
            | Self::Percent
            | Self::Eq
            | Self::EqEq
            | Self::Ne
            | Self::Le
            | Self::Ge
            | Self::LessThan
            | Self::GreaterThan
            | Self::PlusEq
            | Self::MinusEq
            | Self::StarEq
            | Self::SlashEq
            | Self::PercentEq
            | Self::AndAnd
            | Self::OrOr
            | Self::Bang
            | Self::Ampersand
            | Self::Pipe
            | Self::Caret
            | Self::LeftShift
            | Self::RightShift
            | Self::PlusPlus
            | Self::MinusMinus
            | Self::Arrow
            | Self::FatArrow => UniversalTokenRole::Operator,
            Self::LeftParen
            | Self::RightParen
            | Self::LeftBracket
            | Self::RightBracket
            | Self::LeftBrace
            | Self::RightBrace
            | Self::Colon
            | Self::Semicolon
            | Self::Dot
            | Self::Comma
            | Self::DotDot
            | Self::DotDotDot
            | Self::Question
            | Self::At
            | Self::Hash
            | Self::Dollar
            | Self::Backslash
            | Self::Tilde => UniversalTokenRole::Punctuation,
            k if format!("{:?}", k).ends_with("Kw") => UniversalTokenRole::Keyword,
            _ => UniversalTokenRole::None,
        }
    }
}

impl ElementType for VocSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        UniversalElementRole::None
    }
}
