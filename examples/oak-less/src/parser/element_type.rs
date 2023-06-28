use crate::lexer::LessTokenType;
use oak_core::{ElementType, UniversalElementRole};

/// Less element type
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LessElementType {
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

impl ElementType for LessElementType {
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

impl From<LessTokenType> for LessElementType {
    fn from(token: LessTokenType) -> Self {
        match token {
            _ => Self::SourceFile, // Default
        }
    }
}
