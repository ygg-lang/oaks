use oak_core::{Token, TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type ValaToken = Token<ValaTokenType>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum ValaTokenType {
    // 基础
    Whitespace,
    LineComment,
    BlockComment,
    Eof,
    Error,

    // 字面量
    Identifier,
    StringLiteral,
    CharLiteral,
    IntegerLiteral,
    FloatLiteral,

    // 关键字
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

    // 基本类型关键字
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

    // 操作符
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    PlusPlus,
    MinusMinus,
    Eq,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PercentEq,
    EqEq,
    NotEq,
    GreaterThan,
    LessThan,
    GreaterEq,
    LessEq,
    Ampersand,
    Pipe,
    Caret,
    Tilde,
    Bang,
    AndAnd,
    OrOr,
    LeftShift,
    RightShift,
    LeftShiftEq,
    RightShiftEq,
    Question,
    QuestionQuestion,
    Dot,
    Colon,
    ColonColon,
    Arrow,
    Lambda,

    // 标点符号
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Semicolon,
    Backslash,
    At,
    Hash,
    Dollar,
}

impl TokenType for ValaTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::LineComment | Self::BlockComment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::LineComment | Self::BlockComment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            Self::Identifier => UniversalTokenRole::Name,
            Self::StringLiteral | Self::CharLiteral | Self::IntegerLiteral | Self::FloatLiteral => UniversalTokenRole::Literal,
            _ if self.is_keyword() => UniversalTokenRole::Keyword,
            _ => UniversalTokenRole::None,
        }
    }
}

impl ValaTokenType {
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::AbstractKw
                | Self::AsKw
                | Self::BaseKw
                | Self::BreakKw
                | Self::CaseKw
                | Self::CatchKw
                | Self::ClassKw
                | Self::ConstKw
                | Self::ConstructKw
                | Self::ContinueKw
                | Self::DefaultKw
                | Self::DelegateKw
                | Self::DeleteKw
                | Self::DoKw
                | Self::ElseKw
                | Self::EnumKw
                | Self::EnsuresKw
                | Self::ErrordomainKw
                | Self::ExternKw
                | Self::FalseKw
                | Self::FinallyKw
                | Self::ForKw
                | Self::ForeachKw
                | Self::GetKw
                | Self::IfKw
                | Self::InKw
                | Self::InlineKw
                | Self::InterfaceKw
                | Self::InternalKw
                | Self::IsKw
                | Self::LockKw
                | Self::NamespaceKw
                | Self::NewKw
                | Self::NullKw
                | Self::OutKw
                | Self::OverrideKw
                | Self::OwnedKw
                | Self::PrivateKw
                | Self::ProtectedKw
                | Self::PublicKw
                | Self::RefKw
                | Self::RequiresKw
                | Self::ReturnKw
                | Self::SetKw
                | Self::SizeofKw
                | Self::StaticKw
                | Self::StructKw
                | Self::SwitchKw
                | Self::ThisKw
                | Self::ThrowKw
                | Self::ThrowsKw
                | Self::TrueKw
                | Self::TryKw
                | Self::TypeofKw
                | Self::UnownedKw
                | Self::UsingKw
                | Self::VarKw
                | Self::VirtualKw
                | Self::VoidKw
                | Self::VolatileKw
                | Self::WeakKw
                | Self::WhileKw
                | Self::YieldKw
                | Self::BoolKw
                | Self::CharKw
                | Self::UcharKw
                | Self::IntKw
                | Self::UintKw
                | Self::ShortKw
                | Self::UshortKw
                | Self::LongKw
                | Self::UlongKw
                | Self::Int8Kw
                | Self::Uint8Kw
                | Self::Int16Kw
                | Self::Uint16Kw
                | Self::Int32Kw
                | Self::Uint32Kw
                | Self::Int64Kw
                | Self::Uint64Kw
                | Self::FloatKw
                | Self::DoubleKw
                | Self::StringKw
        )
    }
}
