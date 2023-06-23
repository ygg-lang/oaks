use oak_core::{ElementType, Parser, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum NixElementType {
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

impl NixElementType {
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

impl ElementType for NixElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::NixTokenType> for NixElementType {
    fn from(token: crate::lexer::token_type::NixTokenType) -> Self {
        match token {
            crate::lexer::token_type::NixTokenType::Whitespace => NixElementType::Whitespace,
            crate::lexer::token_type::NixTokenType::Newline => NixElementType::Newline,
            crate::lexer::token_type::NixTokenType::Comment => NixElementType::Comment,
            crate::lexer::token_type::NixTokenType::String => NixElementType::String,
            crate::lexer::token_type::NixTokenType::Number => NixElementType::Number,
            crate::lexer::token_type::NixTokenType::Boolean => NixElementType::Boolean,
            crate::lexer::token_type::NixTokenType::True => NixElementType::True,
            crate::lexer::token_type::NixTokenType::False => NixElementType::False,
            crate::lexer::token_type::NixTokenType::Null => NixElementType::Null,
            crate::lexer::token_type::NixTokenType::Identifier => NixElementType::Identifier,
            crate::lexer::token_type::NixTokenType::Let => NixElementType::Let,
            crate::lexer::token_type::NixTokenType::In => NixElementType::In,
            crate::lexer::token_type::NixTokenType::If => NixElementType::If,
            crate::lexer::token_type::NixTokenType::Then => NixElementType::Then,
            crate::lexer::token_type::NixTokenType::Else => NixElementType::Else,
            crate::lexer::token_type::NixTokenType::With => NixElementType::With,
            crate::lexer::token_type::NixTokenType::Inherit => NixElementType::Inherit,
            crate::lexer::token_type::NixTokenType::Rec => NixElementType::Rec,
            crate::lexer::token_type::NixTokenType::Import => NixElementType::Import,
            crate::lexer::token_type::NixTokenType::Assert => NixElementType::Assert,
            crate::lexer::token_type::NixTokenType::Or => NixElementType::Or,
            crate::lexer::token_type::NixTokenType::And => NixElementType::And,
            crate::lexer::token_type::NixTokenType::Not => NixElementType::Not,
            crate::lexer::token_type::NixTokenType::Plus => NixElementType::Plus,
            crate::lexer::token_type::NixTokenType::Minus => NixElementType::Minus,
            crate::lexer::token_type::NixTokenType::Star => NixElementType::Star,
            crate::lexer::token_type::NixTokenType::Slash => NixElementType::Slash,
            crate::lexer::token_type::NixTokenType::Percent => NixElementType::Percent,
            crate::lexer::token_type::NixTokenType::Concatenation => NixElementType::Concatenation,
            crate::lexer::token_type::NixTokenType::Update => NixElementType::Update,
            crate::lexer::token_type::NixTokenType::Implication => NixElementType::Implication,
            crate::lexer::token_type::NixTokenType::Equal => NixElementType::Equal,
            crate::lexer::token_type::NixTokenType::NotEqual => NixElementType::NotEqual,
            crate::lexer::token_type::NixTokenType::Less => NixElementType::Less,
            crate::lexer::token_type::NixTokenType::Greater => NixElementType::Greater,
            crate::lexer::token_type::NixTokenType::LessEqual => NixElementType::LessEqual,
            crate::lexer::token_type::NixTokenType::GreaterEqual => NixElementType::GreaterEqual,
            crate::lexer::token_type::NixTokenType::LogicalAnd => NixElementType::LogicalAnd,
            crate::lexer::token_type::NixTokenType::LogicalOr => NixElementType::LogicalOr,
            crate::lexer::token_type::NixTokenType::Assign => NixElementType::Assign,
            crate::lexer::token_type::NixTokenType::Question => NixElementType::Question,
            crate::lexer::token_type::NixTokenType::LeftParen => NixElementType::LeftParen,
            crate::lexer::token_type::NixTokenType::RightParen => NixElementType::RightParen,
            crate::lexer::token_type::NixTokenType::LeftBrace => NixElementType::LeftBrace,
            crate::lexer::token_type::NixTokenType::RightBrace => NixElementType::RightBrace,
            crate::lexer::token_type::NixTokenType::LeftBracket => NixElementType::LeftBracket,
            crate::lexer::token_type::NixTokenType::RightBracket => NixElementType::RightBracket,
            crate::lexer::token_type::NixTokenType::Semicolon => NixElementType::Semicolon,
            crate::lexer::token_type::NixTokenType::Colon => NixElementType::Colon,
            crate::lexer::token_type::NixTokenType::Comma => NixElementType::Comma,
            crate::lexer::token_type::NixTokenType::Dot => NixElementType::Dot,
            crate::lexer::token_type::NixTokenType::At => NixElementType::At,
            crate::lexer::token_type::NixTokenType::Dollar => NixElementType::Dollar,
            crate::lexer::token_type::NixTokenType::Hash => NixElementType::Hash,
            crate::lexer::token_type::NixTokenType::Root => NixElementType::Root,
            crate::lexer::token_type::NixTokenType::Set => NixElementType::Set,
            crate::lexer::token_type::NixTokenType::List => NixElementType::List,
            crate::lexer::token_type::NixTokenType::Lambda => NixElementType::Lambda,
            crate::lexer::token_type::NixTokenType::LetIn => NixElementType::LetIn,
            crate::lexer::token_type::NixTokenType::IfThenElse => NixElementType::IfThenElse,
            crate::lexer::token_type::NixTokenType::AttrPath => NixElementType::AttrPath,
            crate::lexer::token_type::NixTokenType::Binding => NixElementType::Binding,
            crate::lexer::token_type::NixTokenType::Error => NixElementType::Error,
            crate::lexer::token_type::NixTokenType::Eof => NixElementType::Eof,
        }
    }
}
