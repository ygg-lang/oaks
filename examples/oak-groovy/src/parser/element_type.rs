use oak_core::{ElementType, Parser, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GroovyElementType {
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

impl GroovyElementType {
    pub fn is_keyword(&self) -> bool {
        false
    }
}

impl ElementType for GroovyElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::GroovyTokenType> for GroovyElementType {
    fn from(_token: crate::lexer::token_type::GroovyTokenType) -> Self {
        Self::Root
    }
}
