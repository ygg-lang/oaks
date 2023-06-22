use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

/// Groovy 语法节点类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GroovySyntaxKind {
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

impl GroovySyntaxKind {
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

impl TokenType for GroovySyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Identifier => UniversalTokenRole::Name,
            Self::IntLiteral | Self::FloatLiteral | Self::StringLiteral | Self::CharLiteral | Self::BooleanLiteral | Self::NullLiteral => UniversalTokenRole::Literal,
            _ if self.is_keyword() => UniversalTokenRole::Keyword,
            Self::Plus
            | Self::Minus
            | Self::Star
            | Self::Slash
            | Self::Percent
            | Self::Power
            | Self::Assign
            | Self::PlusAssign
            | Self::MinusAssign
            | Self::StarAssign
            | Self::SlashAssign
            | Self::PercentAssign
            | Self::PowerAssign
            | Self::Equal
            | Self::NotEqual
            | Self::Less
            | Self::Greater
            | Self::LessEqual
            | Self::GreaterEqual
            | Self::Spaceship
            | Self::LogicalAnd
            | Self::LogicalOr
            | Self::LogicalNot
            | Self::BitAnd
            | Self::BitOr
            | Self::BitXor
            | Self::BitNot
            | Self::LeftShift
            | Self::RightShift
            | Self::UnsignedRightShift
            | Self::Increment
            | Self::Decrement
            | Self::Question
            | Self::Elvis
            | Self::SafeNavigation => UniversalTokenRole::Operator,
            Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::LeftBrace | Self::RightBrace | Self::Comma | Self::Period | Self::Semicolon | Self::Colon | Self::At => UniversalTokenRole::Punctuation,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }
}

impl ElementType for GroovySyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root | Self::SourceFile => UniversalElementRole::Root,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    fn is_root(&self) -> bool {
        matches!(self, Self::Root | Self::SourceFile)
    }
}
