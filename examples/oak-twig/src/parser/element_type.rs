use oak_core::{ElementType, Parser, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TwigElementType {
    // 节点种类
    Root,
    Document,
    Template,
    Block,
    Variable,
    Filter,
    Function,
    Tag,
    Comment,
    Text,
    Expression,

    // 细分字面量类
    String,
    Number,
    Boolean,
    Null,
    Array,
    Object,
    Identifier,
    Operator,
    ErrorNode,

    // 词法种类
    LeftBrace,         // {
    RightBrace,        // }
    LeftBracket,       // [
    RightBracket,      // ]
    DoubleLeftBrace,   // {{
    DoubleRightBrace,  // }}
    LeftBracePercent,  // {%
    PercentRightBrace, // %}
    LeftParen,         // (
    RightParen,        // )
    Pipe,              // |
    Comma,             // ,
    Dot,               // .
    Colon,             // :
    Semicolon,         // ;
    Eq,                // =
    Plus,              // +
    Minus,             // -
    Star,              // *
    Slash,             // /
    Percent,           // %
    Bang,              // !
    Question,          // ?
    Lt,                // <
    Gt,                // >
    Amp,               // &
    Caret,             // ^
    Tilde,             // ~
    Whitespace,
    Eof,
    Error,
}

impl oak_core::TokenType for TwigElementType {
    type Role = oak_core::UniversalTokenRole;
    const END_OF_STREAM: Self = TwigElementType::Eof;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => oak_core::UniversalTokenRole::Whitespace,
            Self::Comment => oak_core::UniversalTokenRole::Comment,
            _ => oak_core::UniversalTokenRole::None,
        }
    }
}

impl core::fmt::Display for TwigElementType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TwigElementType::Root => f.write_str("Root"),
            TwigElementType::Document => f.write_str("Document"),
            TwigElementType::Template => f.write_str("Template"),
            TwigElementType::Block => f.write_str("Block"),
            TwigElementType::Variable => f.write_str("Variable"),
            TwigElementType::Filter => f.write_str("Filter"),
            TwigElementType::Function => f.write_str("Function"),
            TwigElementType::Tag => f.write_str("Tag"),
            TwigElementType::Comment => f.write_str("Comment"),
            TwigElementType::Text => f.write_str("Text"),
            TwigElementType::Expression => f.write_str("Expression"),
            TwigElementType::String => f.write_str("String"),
            TwigElementType::Number => f.write_str("Number"),
            TwigElementType::Boolean => f.write_str("Boolean"),
            TwigElementType::Null => f.write_str("Null"),
            TwigElementType::Array => f.write_str("Array"),
            TwigElementType::Object => f.write_str("Object"),
            TwigElementType::Identifier => f.write_str("Identifier"),
            TwigElementType::Operator => f.write_str("Operator"),
            TwigElementType::ErrorNode => f.write_str("ErrorNode"),
            TwigElementType::LeftBrace => f.write_str("{"),
            TwigElementType::RightBrace => f.write_str("}"),
            TwigElementType::LeftBracket => f.write_str("["),
            TwigElementType::RightBracket => f.write_str("]"),
            TwigElementType::DoubleLeftBrace => f.write_str("{{"),
            TwigElementType::DoubleRightBrace => f.write_str("}}"),
            TwigElementType::LeftBracePercent => f.write_str("{%"),
            TwigElementType::PercentRightBrace => f.write_str("%}"),
            TwigElementType::LeftParen => f.write_str("("),
            TwigElementType::RightParen => f.write_str(")"),
            TwigElementType::Pipe => f.write_str("|"),
            TwigElementType::Comma => f.write_str(","),
            TwigElementType::Dot => f.write_str("."),
            TwigElementType::Colon => f.write_str(":"),
            TwigElementType::Semicolon => f.write_str(";"),
            TwigElementType::Eq => f.write_str("="),
            TwigElementType::Plus => f.write_str("+"),
            TwigElementType::Minus => f.write_str("-"),
            TwigElementType::Star => f.write_str("*"),
            TwigElementType::Slash => f.write_str("/"),
            TwigElementType::Percent => f.write_str("%"),
            TwigElementType::Bang => f.write_str("!"),
            TwigElementType::Question => f.write_str("?"),
            TwigElementType::Lt => f.write_str("<"),
            TwigElementType::Gt => f.write_str(">"),
            TwigElementType::Amp => f.write_str("&"),
            TwigElementType::Caret => f.write_str("^"),
            TwigElementType::Tilde => f.write_str("~"),
            TwigElementType::Whitespace => f.write_str("Whitespace"),
            TwigElementType::Eof => f.write_str("EOF"),
            TwigElementType::Error => f.write_str("Error"),
        }
    }
}

impl ElementType for TwigElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::TwigTokenType> for TwigElementType {
    fn from(token: crate::lexer::token_type::TwigTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
