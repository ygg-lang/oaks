use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

impl TokenType for ObjectiveCTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Error;

    fn is_ignored(&self) -> bool {
        match self {
            Self::Whitespace | Self::Newline | Self::CommentToken => true,
            _ => false,
        }
    }

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalTokenRole::None,
        }
    }
}

pub type ObjectiveCToken = Token<ObjectiveCTokenType>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ObjectiveCTokenType {
    // 节点类型
    Root,
    InterfaceDeclaration,
    ImplementationDeclaration,
    MethodDeclaration,
    PropertyDeclaration,
    ProtocolDeclaration,
    CategoryDeclaration,
    ClassExtension,

    // 关键字
    InterfaceKeyword,
    ImplementationKeyword,
    EndKeyword,
    PropertyKeyword,
    SynthesizeKeyword,
    DynamicKeyword,
    ProtocolKeyword,
    CategoryKeyword,
    ImportKeyword,
    IncludeKeyword,
    IfKeyword,
    ElseKeyword,
    ForKeyword,
    WhileKeyword,
    DoKeyword,
    SwitchKeyword,
    CaseKeyword,
    DefaultKeyword,
    BreakKeyword,
    ContinueKeyword,
    ReturnKeyword,
    VoidKeyword,
    IntKeyword,
    FloatKeyword,
    DoubleKeyword,
    CharKeyword,
    BoolKeyword,
    IdKeyword,
    SelfKeyword,
    SuperKeyword,
    NilKeyword,
    YesKeyword,
    NoKeyword,

    // 符号
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Semicolon,
    Comma,
    Dot,
    Colon,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
    Not,
    Question,
    At,

    // 字面量
    Identifier,
    IntegerLiteral,
    FloatLiteral,
    String,
    Character,

    // 其他
    Whitespace,
    Newline,
    CommentToken,
    Error,
    Eof,
}
