use oak_core::{Token, TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type VLangToken = Token<VLangTokenType>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum VLangTokenType {
    // Basic
    Identifier,
    Whitespace,
    Newline,
    Comment,
    Error,
    Eof,

    // Literals
    StringLiteral,
    CharLiteral,
    IntegerLiteral,
    FloatLiteral,
    BoolLiteral,

    // Keywords
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

    // Operators and Punctuation
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PercentEq,
    PlusPlus,
    MinusMinus,
    Ampersand,
    Pipe,
    Caret,
    AmpersandEq,
    PipeEq,
    CaretEq,
    AndAnd,
    OrOr,
    Eq,
    EqEq,
    Ne,
    Bang,
    Lt,
    Le,
    Gt,
    Ge,
    LeftShift,
    RightShift,
    LeftShiftEq,
    RightShiftEq,
    Arrow,
    FatArrow,
    LessThan,    // Used in lexer/mod.rs:473
    GreaterThan, // Used in lexer/mod.rs:493
    DotDot,
    DotDotDot,
    Comma,
    Semicolon,
    Dot,
    Colon,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Question,
    Tilde,
}

impl TokenType for VLangTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}
