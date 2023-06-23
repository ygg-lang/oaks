use oak_core::{Token, TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type VomlToken = Token<VomlTokenType>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum VomlTokenType {
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
    BoolLiteral,

    // Basic kinds
    Identifier,
    Number,
    String,
    Whitespace,
    Comment,
    Error,
    Eof,

    // Literals used by the lexer
    StringLiteral,
    CharLiteral,
    FloatLiteral,
    IntegerLiteral,

    // Operators and punctuation
    Plus,
    PlusEq,
    PlusPlus,
    Minus,
    MinusEq,
    MinusMinus,
    Arrow,
    Star,
    StarEq,
    Slash,
    SlashEq,
    Percent,
    PercentEq,
    Ampersand,
    AmpersandEq,
    AndAnd,
    Pipe,
    PipeEq,
    OrOr,
    Caret,
    CaretEq,
    Eq,
    EqEq,
    FatArrow,
    Bang,
    Ne,
    LessThan,
    Le,
    LeftShift,
    LeftShiftEq,
    GreaterThan,
    Ge,
    RightShift,
    RightShiftEq,
    Dot,
    DotDot,
    DotDotDot,
    Comma,
    Colon,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Question,
    Tilde,
}

impl TokenType for VomlTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            Self::ModuleKw
            | Self::ImportKw
            | Self::PubKw
            | Self::FnKw
            | Self::StructKw
            | Self::InterfaceKw
            | Self::EnumKw
            | Self::TypeKw
            | Self::ConstKw
            | Self::MutKw
            | Self::SharedKw
            | Self::VolatileKw
            | Self::UnsafeKw
            | Self::IfKw
            | Self::ElseKw
            | Self::ForKw
            | Self::InKw
            | Self::MatchKw
            | Self::OrKw
            | Self::ReturnKw
            | Self::BreakKw
            | Self::ContinueKw
            | Self::GotoKw
            | Self::DeferKw
            | Self::GoKw
            | Self::SelectKw
            | Self::LockKw
            | Self::RlockKw
            | Self::AsKw
            | Self::IsKw
            | Self::SizeofKw
            | Self::TypeofKw
            | Self::OffsetofKw
            | Self::AssertKw
            | Self::PanicKw
            | Self::EprintlnKw
            | Self::PrintlnKw
            | Self::PrintKw
            | Self::EprintKw
            | Self::BoolKw
            | Self::I8Kw
            | Self::I16Kw
            | Self::I32Kw
            | Self::I64Kw
            | Self::U8Kw
            | Self::U16Kw
            | Self::U32Kw
            | Self::U64Kw
            | Self::IntKw
            | Self::UintKw
            | Self::F32Kw
            | Self::F64Kw
            | Self::StringKw
            | Self::RuneKw
            | Self::ByteKw
            | Self::VoidptrKw
            | Self::CharKw => UniversalTokenRole::Keyword,
            Self::BoolLiteral | Self::StringLiteral | Self::CharLiteral | Self::FloatLiteral | Self::IntegerLiteral => UniversalTokenRole::Literal,
            Self::Identifier => UniversalTokenRole::Name,
            Self::Number => UniversalTokenRole::Literal,
            Self::String => UniversalTokenRole::Literal,
            Self::Plus
            | Self::PlusEq
            | Self::PlusPlus
            | Self::Minus
            | Self::MinusEq
            | Self::MinusMinus
            | Self::Arrow
            | Self::Star
            | Self::StarEq
            | Self::Slash
            | Self::SlashEq
            | Self::Percent
            | Self::PercentEq
            | Self::Ampersand
            | Self::AmpersandEq
            | Self::AndAnd
            | Self::Pipe
            | Self::PipeEq
            | Self::OrOr
            | Self::Caret
            | Self::CaretEq
            | Self::Eq
            | Self::EqEq
            | Self::FatArrow
            | Self::Bang
            | Self::Ne
            | Self::LessThan
            | Self::Le
            | Self::LeftShift
            | Self::LeftShiftEq
            | Self::GreaterThan
            | Self::Ge
            | Self::RightShift
            | Self::RightShiftEq
            | Self::Dot
            | Self::DotDot
            | Self::DotDotDot
            | Self::Question
            | Self::Tilde => UniversalTokenRole::Operator,
            Self::Comma | Self::Colon | Self::Semicolon => UniversalTokenRole::Punctuation,
            Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::LeftBrace | Self::RightBrace => UniversalTokenRole::Punctuation,
        }
    }
}
