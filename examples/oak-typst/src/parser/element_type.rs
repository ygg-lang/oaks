use oak_core::{ElementType, Parser, UniversalElementRole};

/// Represents an element type in a Typst document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TypstElementType {
    /// Root node of the AST.
    Root,
    /// A heading element.
    Heading,
    /// A quote element.
    Quote,
    /// A math element.
    Math,
    /// Strong text.
    Strong,
    /// Emphasized text.
    Emphasis,
    /// A list item.
    ListItem,
    /// An enumeration item.
    EnumItem,
    /// Raw content (e.g., code blocks).
    Raw,
}

impl ElementType for TypstElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::Heading => UniversalElementRole::Statement,
            Self::Math => UniversalElementRole::Expression,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::TypstTokenType> for TypstElementType {
    fn from(token: crate::lexer::token_type::TypstTokenType) -> Self {
        use crate::lexer::token_type::TypstTokenType::*;
        match token {
            Heading => Self::Heading,
            Quote => Self::Quote,
            Math | InlineMath | DisplayMath => Self::Math,
            Strong => Self::Strong,
            Emphasis => Self::Emphasis,
            ListItem => Self::ListItem,
            EnumItem => Self::EnumItem,
            Raw | Code => Self::Raw,
            _ => Self::Root,
        }
    }
}
