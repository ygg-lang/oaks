use oak_core::{ElementType, Token, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::Serialize;

pub type ObjectiveCToken = Token<ObjectiveCSyntaxKind>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, serde::Deserialize)]
pub enum ObjectiveCSyntaxKind {
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

impl TokenType for ObjectiveCSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::CommentToken => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }
}

impl ElementType for ObjectiveCSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}
