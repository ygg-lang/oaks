use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type GroovyToken = Token<GroovyTokenType>;

impl GroovyTokenType {
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::AbstractKeyword
                | Self::AsKeyword
                | Self::AssertKeyword
                | Self::BreakKeyword
                | Self::CaseKeyword
                | Self::CatchKeyword
                | Self::ClassKeyword
                | Self::ConstKeyword
                | Self::ContinueKeyword
                | Self::DefKeyword
                | Self::DefaultKeyword
                | Self::DoKeyword
                | Self::ElseKeyword
                | Self::EnumKeyword
                | Self::ExtendsKeyword
                | Self::FinalKeyword
                | Self::FinallyKeyword
                | Self::ForKeyword
                | Self::GotoKeyword
                | Self::IfKeyword
                | Self::ImplementsKeyword
                | Self::ImportKeyword
                | Self::InKeyword
                | Self::InstanceofKeyword
                | Self::InterfaceKeyword
                | Self::NativeKeyword
                | Self::NewKeyword
                | Self::PackageKeyword
                | Self::PrivateKeyword
                | Self::ProtectedKeyword
                | Self::PublicKeyword
                | Self::ReturnKeyword
                | Self::StaticKeyword
                | Self::StrictfpKeyword
                | Self::SuperKeyword
                | Self::SwitchKeyword
                | Self::SynchronizedKeyword
                | Self::ThisKeyword
                | Self::ThrowKeyword
                | Self::ThrowsKeyword
                | Self::TraitKeyword
                | Self::TransientKeyword
                | Self::TryKeyword
                | Self::VoidKeyword
                | Self::VolatileKeyword
                | Self::WhileKeyword
        )
    }
}

impl TokenType for GroovyTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Error;

    fn is_ignored(&self) -> bool {
        false
    }

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalTokenRole::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GroovyTokenType {
    Root,
    SourceFile,
    // 字面量
    IntLiteral,
    FloatLiteral,
    StringLiteral,
    CharLiteral,
    BooleanLiteral,
    NullLiteral,

    // 标识符
    Identifier,

    // 关键字
    AbstractKeyword,
    AsKeyword,
    AssertKeyword,
    BreakKeyword,
    CaseKeyword,
    CatchKeyword,
    ClassKeyword,
    ConstKeyword,
    ContinueKeyword,
    DefKeyword,
    DefaultKeyword,
    DoKeyword,
    ElseKeyword,
    EnumKeyword,
    ExtendsKeyword,
    FinalKeyword,
    FinallyKeyword,
    ForKeyword,
    GotoKeyword,
    IfKeyword,
    ImplementsKeyword,
    ImportKeyword,
    InKeyword,
    InstanceofKeyword,
    InterfaceKeyword,
    NativeKeyword,
    NewKeyword,
    PackageKeyword,
    PrivateKeyword,
    ProtectedKeyword,
    PublicKeyword,
    ReturnKeyword,
    StaticKeyword,
    StrictfpKeyword,
    SuperKeyword,
    SwitchKeyword,
    SynchronizedKeyword,
    ThisKeyword,
    ThrowKeyword,
    ThrowsKeyword,
    TraitKeyword,
    TransientKeyword,
    TryKeyword,
    VoidKeyword,
    VolatileKeyword,
    WhileKeyword,

    // 操作符
    Plus,    // +
    Minus,   // -
    Star,    // *
    Slash,   // /
    Percent, // %
    Power,   // **

    Assign,        // =
    PlusAssign,    // +=
    MinusAssign,   // -=
    StarAssign,    // *=
    SlashAssign,   // /=
    PercentAssign, // %=
    PowerAssign,   // **=

    Equal,        // ==
    NotEqual,     // !=
    Less,         // <
    Greater,      // >
    LessEqual,    // <=
    GreaterEqual, // >=
    Spaceship,    // <=>

    LogicalAnd, // &&
    LogicalOr,  // ||
    LogicalNot, // !

    BitAnd,             // &
    BitOr,              // |
    BitXor,             // ^
    BitNot,             // ~
    LeftShift,          // <<
    RightShift,         // >>
    UnsignedRightShift, // >>>

    Increment, // ++
    Decrement, // --

    Question,       // ?
    Colon,          // :
    Elvis,          // ?:
    SafeNavigation, // ?.

    // 分隔符
    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    LeftBrace,    // {
    RightBrace,   // }
    Comma,        // ,
    Period,       // .
    Semicolon,    // ;
    At,           // @

    // 空白和注释
    Whitespace,
    Comment,

    // 特殊
    Newline,
    Eof,
    Error,
}
