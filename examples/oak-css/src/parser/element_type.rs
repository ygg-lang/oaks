use crate::lexer::CssTokenType;
use oak_core::{ElementType, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// CSS element type
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CssElementType {
    /// Source file (root)
    SourceFile,
    /// Rule set
    RuleSet,
    /// Selector list
    SelectorList,
    /// Selector
    Selector,
    /// Declaration block
    DeclarationBlock,
    /// Declaration
    Declaration,
    /// Property
    Property,
    /// Value
    Value,
    /// At-rule
    AtRule,
    /// Media query
    MediaQuery,
    /// Function
    Function,
    /// Url
    Url,
    /// Calc expression
    CalcExpression,
}

impl ElementType for CssElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::SourceFile => UniversalElementRole::Root,
            Self::RuleSet => UniversalElementRole::Container,
            Self::SelectorList => UniversalElementRole::Detail,
            Self::DeclarationBlock => UniversalElementRole::Container,
            Self::Declaration => UniversalElementRole::Statement,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<CssTokenType> for CssElementType {
    fn from(token: CssTokenType) -> Self {
        match token {
            _ => Self::SourceFile, // Default
        }
    }
}
