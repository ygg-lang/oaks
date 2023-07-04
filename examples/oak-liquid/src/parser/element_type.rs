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
    /// Assign statement for variable assignment
    Assign,
    /// Capture block for capturing output into a variable
    Capture,
    /// Case statement for switch-like conditional
    Case,
    /// Include statement for template inclusion
    Include,
    /// Render statement for rendering a snippet (Liquid 5)
    Render,
    /// Unless statement for negated conditional
    Unless,
    /// Raw block for unprocessed content
    Raw,
    /// Break statement for loop control
    Break,
    /// Continue statement for loop control
    Continue,
    /// Tablerow statement for table iteration
    Tablerow,
    /// Cycle statement for cycling through values
    Cycle,
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
            crate::lexer::token_type::LiquidTokenType::Eof => Self::Error,
            crate::lexer::token_type::LiquidTokenType::Error => Self::Error,
            crate::lexer::token_type::LiquidTokenType::EqEq => Self::Expression,
            crate::lexer::token_type::LiquidTokenType::Neq => Self::Expression,
            crate::lexer::token_type::LiquidTokenType::LtEq => Self::Expression,
            crate::lexer::token_type::LiquidTokenType::GtEq => Self::Expression,
            crate::lexer::token_type::LiquidTokenType::DotDot => Self::Expression,
            crate::lexer::token_type::LiquidTokenType::And => Self::Expression,
            crate::lexer::token_type::LiquidTokenType::Or => Self::Expression,
            crate::lexer::token_type::LiquidTokenType::Not => Self::Expression,
            crate::lexer::token_type::LiquidTokenType::TrimMark => Self::Tag,
            _ => Self::Expression,
        }
    }
}
