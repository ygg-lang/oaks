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
                match token {
            crate::lexer::token_type::TailwindTokenType::Root => Self::Root,
            crate::lexer::token_type::TailwindTokenType::Document => Self::Document,
            crate::lexer::token_type::TailwindTokenType::Template => Self::Template,
            crate::lexer::token_type::TailwindTokenType::Block => Self::Block,
            crate::lexer::token_type::TailwindTokenType::Variable => Self::Variable,
            crate::lexer::token_type::TailwindTokenType::Filter => Self::Filter,
            crate::lexer::token_type::TailwindTokenType::Function => Self::Function,
            crate::lexer::token_type::TailwindTokenType::Tag => Self::Tag,
            crate::lexer::token_type::TailwindTokenType::Comment => Self::Comment,
            crate::lexer::token_type::TailwindTokenType::Text => Self::Text,
            crate::lexer::token_type::TailwindTokenType::Expression => Self::Expression,
            crate::lexer::token_type::TailwindTokenType::String => Self::String,
            crate::lexer::token_type::TailwindTokenType::Number => Self::Number,
            crate::lexer::token_type::TailwindTokenType::Boolean => Self::Boolean,
            crate::lexer::token_type::TailwindTokenType::Null => Self::Null,
            crate::lexer::token_type::TailwindTokenType::Array => Self::Array,
            crate::lexer::token_type::TailwindTokenType::Object => Self::Object,
            crate::lexer::token_type::TailwindTokenType::Identifier => Self::Identifier,
            crate::lexer::token_type::TailwindTokenType::Operator => Self::Operator,
            crate::lexer::token_type::TailwindTokenType::ErrorNode => Self::ErrorNode,
            crate::lexer::token_type::TailwindTokenType::LeftBrace => Self::LeftBrace,
            crate::lexer::token_type::TailwindTokenType::RightBrace => Self::RightBrace,
            crate::lexer::token_type::TailwindTokenType::LeftBracket => Self::LeftBracket,
            crate::lexer::token_type::TailwindTokenType::RightBracket => Self::RightBracket,
            crate::lexer::token_type::TailwindTokenType::DoubleLeftBrace => Self::DoubleLeftBrace,
            crate::lexer::token_type::TailwindTokenType::DoubleRightBrace => Self::DoubleRightBrace,
            crate::lexer::token_type::TailwindTokenType::LeftBracePercent => Self::LeftBracePercent,
            crate::lexer::token_type::TailwindTokenType::PercentRightBrace => Self::PercentRightBrace,
            crate::lexer::token_type::TailwindTokenType::LeftParen => Self::LeftParen,
            crate::lexer::token_type::TailwindTokenType::RightParen => Self::RightParen,
            crate::lexer::token_type::TailwindTokenType::Pipe => Self::Pipe,
            crate::lexer::token_type::TailwindTokenType::Comma => Self::Comma,
            crate::lexer::token_type::TailwindTokenType::Dot => Self::Dot,
            crate::lexer::token_type::TailwindTokenType::Colon => Self::Colon,
            crate::lexer::token_type::TailwindTokenType::Semicolon => Self::Semicolon,
            crate::lexer::token_type::TailwindTokenType::Eq => Self::Eq,
            crate::lexer::token_type::TailwindTokenType::Plus => Self::Plus,
            crate::lexer::token_type::TailwindTokenType::Minus => Self::Minus,
            crate::lexer::token_type::TailwindTokenType::Star => Self::Star,
            crate::lexer::token_type::TailwindTokenType::Slash => Self::Slash,
            crate::lexer::token_type::TailwindTokenType::Percent => Self::Percent,
            crate::lexer::token_type::TailwindTokenType::Bang => Self::Bang,
            crate::lexer::token_type::TailwindTokenType::Question => Self::Question,
            crate::lexer::token_type::TailwindTokenType::Lt => Self::Lt,
            crate::lexer::token_type::TailwindTokenType::Gt => Self::Gt,
            crate::lexer::token_type::TailwindTokenType::Amp => Self::Amp,
            crate::lexer::token_type::TailwindTokenType::Caret => Self::Caret,
            crate::lexer::token_type::TailwindTokenType::Tilde => Self::Tilde,
            crate::lexer::token_type::TailwindTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::TailwindTokenType::Eof => Self::Eof,
            crate::lexer::token_type::TailwindTokenType::Error => Self::Error,
            _ => Self::Error,
        }
    }
}
