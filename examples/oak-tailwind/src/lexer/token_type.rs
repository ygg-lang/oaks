use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type TailwindToken = Token<TailwindTokenType>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TailwindTokenType {
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

impl core::fmt::Display for TailwindTokenType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TailwindTokenType::Root => f.write_str("Root"),
            TailwindTokenType::Document => f.write_str("Document"),
            TailwindTokenType::Template => f.write_str("Template"),
            TailwindTokenType::Block => f.write_str("Block"),
            TailwindTokenType::Variable => f.write_str("Variable"),
            TailwindTokenType::Filter => f.write_str("Filter"),
            TailwindTokenType::Function => f.write_str("Function"),
            TailwindTokenType::Tag => f.write_str("Tag"),
            TailwindTokenType::Comment => f.write_str("Comment"),
            TailwindTokenType::Text => f.write_str("Text"),
            TailwindTokenType::Expression => f.write_str("Expression"),
            TailwindTokenType::String => f.write_str("String"),
            TailwindTokenType::Number => f.write_str("Number"),
            TailwindTokenType::Boolean => f.write_str("Boolean"),
            TailwindTokenType::Null => f.write_str("Null"),
            TailwindTokenType::Array => f.write_str("Array"),
            TailwindTokenType::Object => f.write_str("Object"),
            TailwindTokenType::Identifier => f.write_str("Identifier"),
            TailwindTokenType::Operator => f.write_str("Operator"),
            TailwindTokenType::ErrorNode => f.write_str("ErrorNode"),
            TailwindTokenType::LeftBrace => f.write_str("{"),
            TailwindTokenType::RightBrace => f.write_str("}"),
            TailwindTokenType::LeftBracket => f.write_str("["),
            TailwindTokenType::RightBracket => f.write_str("]"),
            TailwindTokenType::DoubleLeftBrace => f.write_str("{{"),
            TailwindTokenType::DoubleRightBrace => f.write_str("}}"),
            TailwindTokenType::LeftBracePercent => f.write_str("{%"),
            TailwindTokenType::PercentRightBrace => f.write_str("%}"),
            TailwindTokenType::LeftParen => f.write_str("("),
            TailwindTokenType::RightParen => f.write_str(")"),
            TailwindTokenType::Pipe => f.write_str("|"),
            TailwindTokenType::Comma => f.write_str(","),
            TailwindTokenType::Dot => f.write_str("."),
            TailwindTokenType::Colon => f.write_str(":"),
            TailwindTokenType::Semicolon => f.write_str(";"),
            TailwindTokenType::Eq => f.write_str("="),
            TailwindTokenType::Plus => f.write_str("+"),
            TailwindTokenType::Minus => f.write_str("-"),
            TailwindTokenType::Star => f.write_str("*"),
            TailwindTokenType::Slash => f.write_str("/"),
            TailwindTokenType::Percent => f.write_str("%"),
            TailwindTokenType::Bang => f.write_str("!"),
            TailwindTokenType::Question => f.write_str("?"),
            TailwindTokenType::Lt => f.write_str("<"),
            TailwindTokenType::Gt => f.write_str(">"),
            TailwindTokenType::Amp => f.write_str("&"),
            TailwindTokenType::Caret => f.write_str("^"),
            TailwindTokenType::Tilde => f.write_str("~"),
            TailwindTokenType::Whitespace => f.write_str("Whitespace"),
            TailwindTokenType::Eof => f.write_str("Eof"),
            TailwindTokenType::Error => f.write_str("Error"),
        }
    }
}

impl TokenType for TailwindTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}
