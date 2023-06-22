use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NixSyntaxKind {
    // 基础 kind
    Whitespace,
    Newline,
    Comment,
    String,
    Number,
    Boolean,
    True,
    False,
    Null,
    Identifier,

    // 关键
    Let,
    In,
    If,
    Then,
    Else,
    With,
    Inherit,
    Rec,
    Import,
    Assert,
    Or,
    And,
    Not,

    // 操作
    Plus,          // +
    Minus,         // -
    Star,          // *
    Slash,         // /
    Percent,       // %
    Concatenation, // ++
    Update,        // //
    Implication,   // ->
    Equal,         // ==
    NotEqual,      // !=
    Less,          // <
    Greater,       // >
    LessEqual,     // <=
    GreaterEqual,  // >=
    LogicalAnd,    // &&
    LogicalOr,     // ||
    Assign,        // =
    Question,      // ?

    // 分隔
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Semicolon,    // ;
    Colon,        // :
    Comma,        // ,
    Dot,          // .
    At,           // @
    Dollar,       // $
    Hash,         // #

    // Element kinds
    Root,
    Set,
    List,
    Lambda,
    LetIn,
    IfThenElse,
    AttrPath,
    Binding,

    // 特殊
    Error,
    Eof,
}

impl NixSyntaxKind {
    pub fn is_element(&self) -> bool {
        matches!(self, Self::Root | Self::Set | Self::List | Self::Lambda | Self::LetIn | Self::IfThenElse | Self::AttrPath | Self::Binding)
    }

    pub fn is_token(&self) -> bool {
        !self.is_element()
    }

    pub fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }
}

impl TokenType for NixSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
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

impl ElementType for NixSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalElementRole::Error,
            Self::Root => UniversalElementRole::Root,
            Self::Binding | Self::Set | Self::Lambda => UniversalElementRole::Detail,
            _ => UniversalElementRole::None,
        }
    }
}
