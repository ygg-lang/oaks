use oak_core::language::{ElementType, UniversalElementRole};

/// CST element kinds for Metis island language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetisElementType {
    /// File / module root.
    Root,
    /// Island declaration.
    IslandDecl,
    /// Node declaration.
    NodeDecl,
    /// Axiom declaration.
    AxiomDecl,
    /// Connection declaration.
    ConnectionDecl,
    /// Fallback / error node.
    Error,
}

impl ElementType for MetisElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::IslandDecl | Self::NodeDecl | Self::AxiomDecl | Self::ConnectionDecl => UniversalElementRole::Statement,
            Self::Error => UniversalElementRole::Error,
        }
    }
}

impl From<crate::lexer::token_type::MetisTokenType> for MetisElementType {
    fn from(token: crate::lexer::token_type::MetisTokenType) -> Self {
        use crate::lexer::token_type::MetisTokenType as T;
        match token {
            T::KwIsland | T::KwTheorem | T::KwRewrites | T::KwAction => Self::IslandDecl,
            T::KwNode => Self::NodeDecl,
            T::KwAxiom => Self::AxiomDecl,
            T::KwConnection => Self::ConnectionDecl,
            _ => Self::Error,
        }
    }
}
