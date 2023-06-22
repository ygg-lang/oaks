use crate::lexer::ClojureTokenType;
use oak_core::{ElementType, UniversalElementRole};
use serde::{Deserialize, Serialize};

/// Represents all possible element kinds in the Clojure programming language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ClojureElementType {
    /// A wrapper for tokens
    Token(ClojureTokenType),
    /// Root node
    Root,
    /// Root node of the source file
    SourceFile,
    /// List node
    List,
    /// Vector node
    Vector,
    /// Map node
    Map,
    /// Set node
    Set,
    /// Anonymous function node
    AnonFn,
    /// Error node
    Error,
}

impl From<ClojureTokenType> for ClojureElementType {
    fn from(token: ClojureTokenType) -> Self {
        Self::Token(token)
    }
}

impl ElementType for ClojureElementType {
    type Role = UniversalElementRole;

    fn is_root(&self) -> bool {
        matches!(self, Self::SourceFile)
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::SourceFile | Self::Root => UniversalElementRole::Root,
            Self::List | Self::Vector | Self::Map | Self::Set | Self::AnonFn => UniversalElementRole::Container,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}
