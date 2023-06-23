use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type TwigToken = Token<TwigTokenType>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TwigTokenType {
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

impl oak_core::TokenType for TwigTokenType {
    type Role = oak_core::UniversalTokenRole;
    const END_OF_STREAM: Self = TwigTokenType::Eof;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => oak_core::UniversalTokenRole::Whitespace,
            Self::Comment => oak_core::UniversalTokenRole::Comment,
            _ => oak_core::UniversalTokenRole::None,
        }
    }
}

impl core::fmt::Display for TwigTokenType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TwigTokenType::Root => f.write_str("Root"),
            TwigTokenType::Document => f.write_str("Document"),
            TwigTokenType::Template => f.write_str("Template"),
            TwigTokenType::Block => f.write_str("Block"),
            TwigTokenType::Variable => f.write_str("Variable"),
            TwigTokenType::Filter => f.write_str("Filter"),
            TwigTokenType::Function => f.write_str("Function"),
            TwigTokenType::Tag => f.write_str("Tag"),
            TwigTokenType::Comment => f.write_str("Comment"),
            TwigTokenType::Text => f.write_str("Text"),
            TwigTokenType::Expression => f.write_str("Expression"),
            TwigTokenType::String => f.write_str("String"),
            TwigTokenType::Number => f.write_str("Number"),
            TwigTokenType::Boolean => f.write_str("Boolean"),
            TwigTokenType::Null => f.write_str("Null"),
            TwigTokenType::Array => f.write_str("Array"),
            TwigTokenType::Object => f.write_str("Object"),
            TwigTokenType::Identifier => f.write_str("Identifier"),
            TwigTokenType::Operator => f.write_str("Operator"),
            TwigTokenType::ErrorNode => f.write_str("ErrorNode"),
            TwigTokenType::LeftBrace => f.write_str("{"),
            TwigTokenType::RightBrace => f.write_str("}"),
            TwigTokenType::LeftBracket => f.write_str("["),
            TwigTokenType::RightBracket => f.write_str("]"),
            TwigTokenType::DoubleLeftBrace => f.write_str("{{"),
            TwigTokenType::DoubleRightBrace => f.write_str("}}"),
            TwigTokenType::LeftBracePercent => f.write_str("{%"),
            TwigTokenType::PercentRightBrace => f.write_str("%}"),
            TwigTokenType::LeftParen => f.write_str("("),
            TwigTokenType::RightParen => f.write_str(")"),
            TwigTokenType::Pipe => f.write_str("|"),
            TwigTokenType::Comma => f.write_str(","),
            TwigTokenType::Dot => f.write_str("."),
            TwigTokenType::Colon => f.write_str(":"),
            TwigTokenType::Semicolon => f.write_str(";"),
            TwigTokenType::Eq => f.write_str("="),
            TwigTokenType::Plus => f.write_str("+"),
            TwigTokenType::Minus => f.write_str("-"),
            TwigTokenType::Star => f.write_str("*"),
            TwigTokenType::Slash => f.write_str("/"),
            TwigTokenType::Percent => f.write_str("%"),
            TwigTokenType::Bang => f.write_str("!"),
            TwigTokenType::Question => f.write_str("?"),
            TwigTokenType::Lt => f.write_str("<"),
            TwigTokenType::Gt => f.write_str(">"),
            TwigTokenType::Amp => f.write_str("&"),
            TwigTokenType::Caret => f.write_str("^"),
            TwigTokenType::Tilde => f.write_str("~"),
            TwigTokenType::Whitespace => f.write_str("Whitespace"),
            TwigTokenType::Eof => f.write_str("EOF"),
            TwigTokenType::Error => f.write_str("Error"),
        }
    }
}
