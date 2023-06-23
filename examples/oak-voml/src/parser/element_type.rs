use oak_core::{ElementType, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum VomlElementType {
    Root,
    SourceFile,
    Module,
    Function,
    Memory,
    Export,
    Import,
    Param,
    Result,
    Local,
    Instruction,

    // Basic types
    Int,
    Uint,
    F32,
    F64,
    String,
    Rune,
    Byte,
    Voidptr,
    Char,
    Bool,

    // Values
    Identifier,
    Number,
    Boolean,

    // Punctuation and symbols
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Dot,
    Comma,
    Colon,
    Semicolon,

    Whitespace,
    Comment,
    Error,
    Eof,
}

impl ElementType for VomlElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root | Self::SourceFile => UniversalElementRole::Root,
            Self::Module | Self::Function | Self::Memory | Self::Export | Self::Import => UniversalElementRole::Statement,
            Self::Int | Self::Uint | Self::F32 | Self::F64 | Self::String | Self::Rune | Self::Byte | Self::Voidptr | Self::Char | Self::Bool => UniversalElementRole::Typing,
            Self::Identifier => UniversalElementRole::Reference,
            Self::Number | Self::Boolean => UniversalElementRole::Value,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::VomlTokenType> for VomlElementType {
    fn from(token: crate::lexer::token_type::VomlTokenType) -> Self {
        use crate::lexer::token_type::VomlTokenType as T;
        match token {
            T::IntKw => VomlElementType::Int,
            T::UintKw => VomlElementType::Uint,
            T::F32Kw => VomlElementType::F32,
            T::F64Kw => VomlElementType::F64,
            T::StringKw => VomlElementType::String,
            T::RuneKw => VomlElementType::Rune,
            T::ByteKw => VomlElementType::Byte,
            T::VoidptrKw => VomlElementType::Voidptr,
            T::CharKw => VomlElementType::Char,
            T::BoolLiteral => VomlElementType::Boolean,
            T::Identifier => VomlElementType::Identifier,
            T::Number => VomlElementType::Number,
            T::String => VomlElementType::String,
            T::Whitespace => VomlElementType::Whitespace,
            T::Comment => VomlElementType::Comment,
            T::LeftParen => VomlElementType::LeftParen,
            T::RightParen => VomlElementType::RightParen,
            T::LeftBracket => VomlElementType::LeftBracket,
            T::RightBracket => VomlElementType::RightBracket,
            T::LeftBrace => VomlElementType::LeftBrace,
            T::RightBrace => VomlElementType::RightBrace,
            T::Dot => VomlElementType::Dot,
            T::Comma => VomlElementType::Comma,
            T::Colon => VomlElementType::Colon,
            T::Semicolon => VomlElementType::Semicolon,
            T::Error => VomlElementType::Error,
            T::Eof => VomlElementType::Eof,
            _ => VomlElementType::Error,
        }
    }
}
