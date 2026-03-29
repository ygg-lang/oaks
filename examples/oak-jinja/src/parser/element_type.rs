/// Jinja Element Type module
///
/// This module defines the element types for Jinja templates.
use oak_core::{ElementType, UniversalElementRole};

/// Element types for Jinja templates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum JinjaElementType {
    /// Root element
    Root,
    /// Text content
    Text,
    /// Variable expression
    Variable,
    /// Block statement
    Block,
    /// Comment
    Comment,
    /// If statement
    IfStatement,
    /// For loop
    ForStatement,
    /// Macro definition
    MacroDefinition,
    /// Extends statement for template inheritance
    Extends,
    /// Include statement for template inclusion
    Include,
    /// Import statement for module import
    Import,
    /// From-import statement for selective import
    FromImport,
    /// Set statement for variable assignment
    Set,
    /// Do statement for side effects
    Do,
    /// Tag statement
    Tag,
    /// Filter expression
    Filter,
    /// Expression
    Expression,
    /// Identifier
    Identifier,
    /// Literal
    Literal,
    /// Function call
    Function,
    /// Error
    Error,
}

impl ElementType for JinjaElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::JinjaTokenType> for JinjaElementType {
    fn from(token_type: crate::lexer::token_type::JinjaTokenType) -> Self {
        match token_type {
            crate::lexer::token_type::JinjaTokenType::Text => Self::Text,
            crate::lexer::token_type::JinjaTokenType::DoubleLeftBrace => Self::Variable,
            crate::lexer::token_type::JinjaTokenType::DoubleRightBrace => Self::Variable,
            crate::lexer::token_type::JinjaTokenType::LeftBracePercent => Self::Tag,
            crate::lexer::token_type::JinjaTokenType::PercentRightBrace => Self::Tag,
            crate::lexer::token_type::JinjaTokenType::TrimMark => Self::Tag,
            crate::lexer::token_type::JinjaTokenType::Identifier => Self::Identifier,
            crate::lexer::token_type::JinjaTokenType::String => Self::Literal,
            crate::lexer::token_type::JinjaTokenType::Number => Self::Literal,
            crate::lexer::token_type::JinjaTokenType::Boolean => Self::Literal,
            crate::lexer::token_type::JinjaTokenType::Whitespace => Self::Text,
            crate::lexer::token_type::JinjaTokenType::Comment => Self::Comment,
            crate::lexer::token_type::JinjaTokenType::Eof => Self::Error,
            crate::lexer::token_type::JinjaTokenType::Error => Self::Error,
            crate::lexer::token_type::JinjaTokenType::EqEq => Self::Expression,
            crate::lexer::token_type::JinjaTokenType::Neq => Self::Expression,
            crate::lexer::token_type::JinjaTokenType::LtEq => Self::Expression,
            crate::lexer::token_type::JinjaTokenType::GtEq => Self::Expression,
            crate::lexer::token_type::JinjaTokenType::And => Self::Expression,
            crate::lexer::token_type::JinjaTokenType::Or => Self::Expression,
            crate::lexer::token_type::JinjaTokenType::Not => Self::Expression,
            _ => Self::Expression,
        }
    }
}
