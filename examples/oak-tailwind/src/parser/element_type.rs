//! Element types for the Tailwind AST.
use oak_core::{ElementType, UniversalElementRole};

/// Element types for the Tailwind AST.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TailwindElementType {
    /// Root node.
    Root,
    /// A Tailwind class (e.g., `hover:bg-red-500`).
    Class,
    /// A modifier part (e.g., `hover:`).
    Modifier,
    /// A utility part (e.g., `bg-red-500`).
    Utility,
    /// An arbitrary value part (e.g., `[100px]`).
    ArbitraryValue,
    /// A directive (e.g., `@tailwind base`).
    Directive,
    /// A CSS declaration.
    Declaration,
    /// A comment.
    Comment,
    /// Error node.
    ErrorNode,
}

impl core::fmt::Display for TailwindElementType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Root => f.write_str("Root"),
            Self::Class => f.write_str("Class"),
            Self::Modifier => f.write_str("Modifier"),
            Self::Utility => f.write_str("Utility"),
            Self::ArbitraryValue => f.write_str("ArbitraryValue"),
            Self::Directive => f.write_str("Directive"),
            Self::Declaration => f.write_str("Declaration"),
            Self::Comment => f.write_str("Comment"),
            Self::ErrorNode => f.write_str("ErrorNode"),
        }
    }
}

impl ElementType for TailwindElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::Class => UniversalElementRole::Attribute,
            Self::Modifier => UniversalElementRole::AttributeKey,
            Self::Utility => UniversalElementRole::Name,
            Self::ArbitraryValue => UniversalElementRole::Value,
            Self::Directive => UniversalElementRole::Metadata,
            Self::Declaration => UniversalElementRole::Metadata,
            Self::Comment => UniversalElementRole::Documentation,
            Self::ErrorNode => UniversalElementRole::Error,
        }
    }
}

impl From<crate::lexer::token_type::TailwindTokenType> for TailwindElementType {
    fn from(token: crate::lexer::token_type::TailwindTokenType) -> Self {
        match token {
            crate::lexer::token_type::TailwindTokenType::Root => Self::Root,
            crate::lexer::token_type::TailwindTokenType::Directive => Self::Directive,
            crate::lexer::token_type::TailwindTokenType::Modifier => Self::Modifier,
            crate::lexer::token_type::TailwindTokenType::Utility => Self::Utility,
            crate::lexer::token_type::TailwindTokenType::ArbitraryValue => Self::ArbitraryValue,
            crate::lexer::token_type::TailwindTokenType::Comment => Self::Comment,
            _ => Self::ErrorNode,
        }
    }
}
