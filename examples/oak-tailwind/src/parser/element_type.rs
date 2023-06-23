use oak_core::{ElementType, Parser, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TailwindElementType {
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
    String,
    Number,
    Boolean,
    Null,
    Array,
    Object,
    Identifier,
    Operator,
    ErrorNode,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    DoubleLeftBrace,
    DoubleRightBrace,
    LeftBracePercent,
    PercentRightBrace,
    LeftParen,
    RightParen,
    Pipe,
    Comma,
    Dot,
    Colon,
    Semicolon,
    Eq,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Bang,
    Question,
    Lt,
    Gt,
    Amp,
    Caret,
    Tilde,
    Whitespace,
    Eof,
    Error,
}

impl oak_core::TokenType for TailwindElementType {
    type Role = oak_core::UniversalTokenRole;
    const END_OF_STREAM: Self = TailwindElementType::Eof;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => oak_core::UniversalTokenRole::Whitespace,
            Self::Comment => oak_core::UniversalTokenRole::Comment,
            _ => oak_core::UniversalTokenRole::None,
        }
    }
}

impl core::fmt::Display for TailwindElementType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TailwindElementType::Root => f.write_str("Root"),
            TailwindElementType::Document => f.write_str("Document"),
            TailwindElementType::Template => f.write_str("Template"),
            TailwindElementType::Block => f.write_str("Block"),
            TailwindElementType::Variable => f.write_str("Variable"),
            TailwindElementType::Filter => f.write_str("Filter"),
            TailwindElementType::Function => f.write_str("Function"),
            TailwindElementType::Tag => f.write_str("Tag"),
            TailwindElementType::Comment => f.write_str("Comment"),
            TailwindElementType::Text => f.write_str("Text"),
            TailwindElementType::Expression => f.write_str("Expression"),
            TailwindElementType::String => f.write_str("String"),
            TailwindElementType::Number => f.write_str("Number"),
            TailwindElementType::Boolean => f.write_str("Boolean"),
            TailwindElementType::Null => f.write_str("Null"),
            TailwindElementType::Array => f.write_str("Array"),
            TailwindElementType::Object => f.write_str("Object"),
            TailwindElementType::Identifier => f.write_str("Identifier"),
            TailwindElementType::Operator => f.write_str("Operator"),
            TailwindElementType::ErrorNode => f.write_str("ErrorNode"),
            TailwindElementType::LeftBrace => f.write_str("{"),
            TailwindElementType::RightBrace => f.write_str("}"),
            TailwindElementType::LeftBracket => f.write_str("["),
            TailwindElementType::RightBracket => f.write_str("]"),
            TailwindElementType::DoubleLeftBrace => f.write_str("{{"),
            TailwindElementType::DoubleRightBrace => f.write_str("}}"),
            TailwindElementType::LeftBracePercent => f.write_str("{%"),
            TailwindElementType::PercentRightBrace => f.write_str("%}"),
            TailwindElementType::LeftParen => f.write_str("("),
            TailwindElementType::RightParen => f.write_str(")"),
            TailwindElementType::Pipe => f.write_str("|"),
            TailwindElementType::Comma => f.write_str(","),
            TailwindElementType::Dot => f.write_str("."),
            TailwindElementType::Colon => f.write_str(":"),
            TailwindElementType::Semicolon => f.write_str(";"),
            TailwindElementType::Eq => f.write_str("="),
            TailwindElementType::Plus => f.write_str("+"),
            TailwindElementType::Minus => f.write_str("-"),
            TailwindElementType::Star => f.write_str("*"),
            TailwindElementType::Slash => f.write_str("/"),
            TailwindElementType::Percent => f.write_str("%"),
            TailwindElementType::Bang => f.write_str("!"),
            TailwindElementType::Question => f.write_str("?"),
            TailwindElementType::Lt => f.write_str("<"),
            TailwindElementType::Gt => f.write_str(">"),
            TailwindElementType::Amp => f.write_str("&"),
            TailwindElementType::Caret => f.write_str("^"),
            TailwindElementType::Tilde => f.write_str("~"),
            TailwindElementType::Whitespace => f.write_str("Whitespace"),
            TailwindElementType::Eof => f.write_str("Eof"),
            TailwindElementType::Error => f.write_str("Error"),
        }
    }
}

impl ElementType for TailwindElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,

            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::TailwindTokenType> for TailwindElementType {
    fn from(token: crate::lexer::token_type::TailwindTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
