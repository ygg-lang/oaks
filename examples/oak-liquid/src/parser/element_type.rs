/// Liquid Element Type module
///
/// This module defines the element types for Liquid templates.
use oak_core::{ElementType, UniversalElementRole};

/// Element types for Liquid templates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LiquidElementType {
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

impl ElementType for LiquidElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::LiquidTokenType> for LiquidElementType {
    fn from(token_type: crate::lexer::token_type::LiquidTokenType) -> Self {
        match token_type {
            crate::lexer::token_type::LiquidTokenType::Text => Self::Text,
            crate::lexer::token_type::LiquidTokenType::DoubleLeftBrace => Self::Variable,
            crate::lexer::token_type::LiquidTokenType::DoubleRightBrace => Self::Variable,
            crate::lexer::token_type::LiquidTokenType::LeftBracePercent => Self::Tag,
            crate::lexer::token_type::LiquidTokenType::PercentRightBrace => Self::Tag,
            crate::lexer::token_type::LiquidTokenType::Identifier => Self::Identifier,
            crate::lexer::token_type::LiquidTokenType::String => Self::Literal,
            crate::lexer::token_type::LiquidTokenType::Number => Self::Literal,
            crate::lexer::token_type::LiquidTokenType::Boolean => Self::Literal,
            crate::lexer::token_type::LiquidTokenType::Whitespace => Self::Text,
            crate::lexer::token_type::LiquidTokenType::Comment => Self::Comment,
            crate::lexer::token_type::LiquidTokenType::Eof => Self::Text,
            crate::lexer::token_type::LiquidTokenType::Error => Self::Error,
            _ => Self::Expression,
        }
    }
}
