use crate::lexer::JTokenType;
use oak_core::{ElementType, GreenNode, UniversalElementRole};
use std::sync::Arc;

/// Type alias for J syntax tree elements.
pub type JElement<'a> = Arc<GreenNode<'a, JElementType>>;

/// All possible element types in the J syntax tree.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum JElementType {
    /// Root node
    Root,
    /// Compilation unit
    CompilationUnit,

    /// Sentence
    Sentence,

    /// Assignment
    Assignment,

    /// Expression
    Expression,

    /// Verb
    Verb,

    /// Noun
    Noun,

    /// Adverb
    Adverb,

    /// Conjunction
    Conjunction,

    /// Parenthesized expression
    Group,
}

impl From<JTokenType> for JElementType {
    fn from(token: JTokenType) -> Self {
        match token {
            JTokenType::Identifier => Self::Noun,
            JTokenType::NumberLiteral => Self::Noun,
            JTokenType::StringLiteral => Self::Noun,
            JTokenType::IsGlobal | JTokenType::IsLocal => Self::Assignment,
            _ => Self::Expression,
        }
    }
}

impl ElementType for JElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root | Self::CompilationUnit => UniversalElementRole::Root,
            Self::Sentence => UniversalElementRole::Statement,
            Self::Assignment => UniversalElementRole::Binding,
            Self::Expression => UniversalElementRole::Expression,
            _ => UniversalElementRole::None,
        }
    }
}
