#![doc = include_str!("../../readme.md")]

use oak_core::SyntaxKind;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ValkyrieSyntaxKind {
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
    PlusPlus,
    MinusMinus,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PercentEq,
    AndAnd,
    OrOr,
    LeftShift,
    RightShift,
    Arrow,

    // Valkyrie 关键字
    ModKw,
    FnKw,
    StructKw,
    EnumKw,
    TraitKw,
    ImplKw,
    LetKw,
    MutKw,
    ConstKw,
    StaticKw,
    IfKw,
    ElseKw,
    MatchKw,
    ForKw,
    WhileKw,
    LoopKw,
    BreakKw,
    ContinueKw,
    ReturnKw,
    PubKw,
    UseKw,
    AsKw,
    InKw,
    WhereKw,
    SelfKw,
    SuperKw,
    CrateKw,
    TypeKw,
    UnsafeKw,
    ExternKw,
    RefKw,
    MoveKw,
    BoxKw,
    AsyncKw,
    AwaitKw,
    TryKw,
    CatchKw,
    FinallyKw,
    YieldKw,
    MacroKw,
    DynKw,

    // 基本类型
    BoolKw,
    CharKw,
    StrKw,
    I8Kw,
    I16Kw,
    I32Kw,
    I64Kw,
    I128Kw,
    IsizeKw,
    U8Kw,
    U16Kw,
    U32Kw,
    U64Kw,
    U128Kw,
    UsizeKw,
    F32Kw,
    F64Kw,

    // 字面量
    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
    CharLiteral,
    BoolLiteral,

    // 标识符
    Identifier,

    // 注释
    LineComment,
    BlockComment,
}

impl SyntaxKind for ValkyrieSyntaxKind {
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
        true // Valkyrie doesn't have element types in this simple implementation
    }

    fn is_element_type(&self) -> bool {
        false // Valkyrie doesn't have element types in this simple implementation
    }
}
