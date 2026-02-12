//! Voml element types.

use oak_core::{ElementType, UniversalElementRole};

/// Enum representing all possible element types in Voml.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u16)]
pub enum VomlElementType {
    /// Root node of the AST.
    Root,
    /// A source file containing VOML content.
    SourceFile,
    /// A module definition.
    Module,
    /// A function definition.
    Function,
    /// A memory allocation.
    Memory,
    /// An export declaration.
    Export,
    /// An import declaration.
    Import,
    /// A function parameter.
    Param,
    /// A function result.
    Result,
    /// A local variable.
    Local,
    /// A single instruction.
    Instruction,

    /// Integer type (int).
    Int,
    /// Unsigned integer type (uint).
    Uint,
    /// 32-bit floating point type (f32).
    F32,
    /// 64-bit floating point type (f64).
    F64,
    /// String type.
    String,
    /// Rune (Unicode character) type.
    Rune,
    /// Byte type.
    Byte,
    /// Void pointer type.
    Voidptr,
    /// Character type.
    Char,
    /// Boolean type.
    Bool,

    /// An identifier.
    Identifier,
    /// A numeric literal.
    Number,
    /// A boolean literal (true/false).
    Boolean,

    /// Left parenthesis (().
    LeftParen,
    /// Right parenthesis ()).
    RightParen,
    /// Left bracket ([).
    LeftBracket,
    /// Right bracket (]).
    RightBracket,
    /// Left brace ({).
    LeftBrace,
    /// Right brace (}).
    RightBrace,
    /// Dot separator (.).
    Dot,
    /// Comma separator (,).
    Comma,
    /// Colon separator (:).
    Colon,
    /// Semicolon separator (;).
    Semicolon,

    /// Whitespace characters.
    Whitespace,
    /// A comment.
    Comment,
    /// An error node representing a syntax error.
    Error,
    /// End of file marker.
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
        unsafe { std::mem::transmute(token) }
    }
}
