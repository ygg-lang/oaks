use oak_core::{ElementType, UniversalElementRole};

/// Element types for the YAML language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u16)]
pub enum YamlElementType {
    /// A leaf node containing a single token.
    Token(crate::lexer::token_type::YamlTokenType),
    /// A YAML document.
    Document,
    /// The root node of a YAML file.
    Root,
    /// An error node.
    Error,
}

impl ElementType for YamlElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::Document => UniversalElementRole::Container,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::YamlTokenType> for YamlElementType {
    fn from(token: crate::lexer::token_type::YamlTokenType) -> Self {
        Self::Token(token)
    }
}
