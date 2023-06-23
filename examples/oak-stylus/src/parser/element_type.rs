use oak_core::{ElementType, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum StylusElementType {
    // 节点种类
    Root,
    Document,
    Rule,
    Selector,
    Property,
    Value,
    Block,

    // 词法种类
    Identifier,
    Number,
    String,
    Color,
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    Colon,
    Semicolon,
    Comma,
    Dot,
    Hash,
    Ampersand,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    Whitespace,
    Newline,
    Comment,
    Eof,
    Error,
}

impl ElementType for StylusElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            StylusElementType::Root => UniversalElementRole::Root,
            StylusElementType::Document => UniversalElementRole::Container,
            StylusElementType::Rule => UniversalElementRole::Statement,
            StylusElementType::Selector => UniversalElementRole::Binding,
            StylusElementType::Property => UniversalElementRole::AttributeKey,
            StylusElementType::Value => UniversalElementRole::Value,
            StylusElementType::Block => UniversalElementRole::Container,
            _ => UniversalElementRole::Value,
        }
    }
}

impl From<crate::lexer::token_type::StylusTokenType> for StylusElementType {
    fn from(token: crate::lexer::token_type::StylusTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
