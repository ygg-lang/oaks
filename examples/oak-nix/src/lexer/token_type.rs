use oak_core::{Token, TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type NixToken = Token<NixTokenType>;

impl NixTokenType {
    pub fn is_element(&self) -> bool {
        matches!(self, Self::Root | Self::Set | Self::List | Self::Lambda | Self::LetIn | Self::IfThenElse | Self::AttrPath | Self::Binding)
    }

    pub fn is_token(&self) -> bool {
        !self.is_element()
    }

    pub fn is_keyword(&self) -> bool {
        matches!(self, Self::Let | Self::In | Self::If | Self::Then | Self::Else | Self::With | Self::Inherit | Self::Rec | Self::Import | Self::Assert | Self::Or | Self::And | Self::Not)
    }

    pub fn is_operator(&self) -> bool {
        matches!(
            self,
            Self::Plus
                | Self::Minus
                | Self::Star
                | Self::Slash
                | Self::Percent
                | Self::Concatenation
                | Self::Update
                | Self::Implication
                | Self::Equal
                | Self::NotEqual
                | Self::Less
                | Self::Greater
                | Self::LessEqual
                | Self::GreaterEqual
                | Self::LogicalAnd
                | Self::LogicalOr
                | Self::Assign
                | Self::Question
        )
    }

    pub fn is_punctuation(&self) -> bool {
        matches!(self, Self::LeftParen | Self::RightParen | Self::LeftBrace | Self::RightBrace | Self::LeftBracket | Self::RightBracket | Self::Semicolon | Self::Colon | Self::Comma | Self::Dot | Self::At | Self::Dollar | Self::Hash)
    }
}

impl TokenType for NixTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Error;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Identifier => UniversalTokenRole::Name,
            Self::String | Self::Number | Self::Boolean | Self::True | Self::False | Self::Null => UniversalTokenRole::Literal,
            _ if self.is_keyword() => UniversalTokenRole::Keyword,
            _ if self.is_operator() => UniversalTokenRole::Operator,
            _ if self.is_punctuation() => UniversalTokenRole::Punctuation,
            _ => UniversalTokenRole::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum NixTokenType {
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
    At,           // ↯
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
