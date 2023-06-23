use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type StylusToken = Token<StylusTokenType>;

impl TokenType for StylusTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn role(&self) -> Self::Role {
        match self {
            Self::Identifier => UniversalTokenRole::Name,
            Self::Number | Self::String | Self::Color => UniversalTokenRole::Literal,
            Self::LeftBrace | Self::RightBrace | Self::LeftParen | Self::RightParen | Self::Colon | Self::Semicolon | Self::Comma => UniversalTokenRole::Punctuation,
            Self::Dot | Self::Hash | Self::Ampersand | Self::Plus | Self::Minus | Self::Star | Self::Slash | Self::Percent | Self::Equal => UniversalTokenRole::Operator,
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}

impl StylusTokenType {
    /// 检查是否为值类型
    pub fn is_value(self) -> bool {
        matches!(self, StylusTokenType::Number | StylusTokenType::String | StylusTokenType::Color | StylusTokenType::Identifier)
    }

    /// 检查是否为操作符
    pub fn is_operator(self) -> bool {
        matches!(self, StylusTokenType::Plus | StylusTokenType::Minus | StylusTokenType::Star | StylusTokenType::Slash | StylusTokenType::Percent | StylusTokenType::Equal)
    }
}

impl core::fmt::Display for StylusTokenType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let name = match self {
            StylusTokenType::Root => "Root",
            StylusTokenType::Document => "Document",
            StylusTokenType::Rule => "Rule",
            StylusTokenType::Selector => "Selector",
            StylusTokenType::Property => "Property",
            StylusTokenType::Value => "Value",
            StylusTokenType::Block => "Block",
            StylusTokenType::Identifier => "Identifier",
            StylusTokenType::Number => "Number",
            StylusTokenType::String => "String",
            StylusTokenType::Color => "Color",
            StylusTokenType::LeftBrace => "LeftBrace",
            StylusTokenType::RightBrace => "RightBrace",
            StylusTokenType::LeftParen => "LeftParen",
            StylusTokenType::RightParen => "RightParen",
            StylusTokenType::Colon => "Colon",
            StylusTokenType::Semicolon => "Semicolon",
            StylusTokenType::Comma => "Comma",
            StylusTokenType::Dot => "Dot",
            StylusTokenType::Hash => "Hash",
            StylusTokenType::Ampersand => "Ampersand",
            StylusTokenType::Plus => "Plus",
            StylusTokenType::Minus => "Minus",
            StylusTokenType::Star => "Star",
            StylusTokenType::Slash => "Slash",
            StylusTokenType::Percent => "Percent",
            StylusTokenType::Equal => "Equal",
            StylusTokenType::Whitespace => "Whitespace",
            StylusTokenType::Newline => "Newline",
            StylusTokenType::Comment => "Comment",
            StylusTokenType::Eof => "Eof",
            StylusTokenType::Error => "Error",
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum StylusTokenType {
    // 节点种类
    Root,
    Document,
    Rule,
    Selector,
    Property,
    Value,
    Block,

    // 词法种类
    Identifier, // body, div, color, etc.
    Number,     // 10, 100px, 1.5em
    String,     // "Arial", 'Helvetica'
    Color,      // #fff, red, rgb(255,0,0)
    LeftBrace,  // {
    RightBrace, // }
    LeftParen,  // (
    RightParen, // )
    Colon,      // :
    Semicolon,  // ;
    Comma,      // ,
    Dot,        // .
    Hash,       // #
    Ampersand,  // &
    Plus,       // +
    Minus,      // -
    Star,       // *
    Slash,      // /
    Percent,    // %
    Equal,      // =
    Whitespace,
    Newline,
    Comment,
    Eof,
    Error,
}
