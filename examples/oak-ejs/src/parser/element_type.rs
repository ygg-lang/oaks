/// EJS Element Type module
///
/// This module defines the element types for EJS (Embedded JavaScript) templates.
use oak_core::{ElementType, UniversalElementRole};

/// Element types for EJS templates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EjsElementType {
    /// Root element representing the entire template
    Root,
    /// Text content outside of EJS tags
    Text,
    /// Escaped output expression `<%= ... %>`
    OutputEscape,
    /// Raw output expression `<%- ... %>`
    OutputRaw,
    /// Code block `<% ... %>`
    Code,
    /// Comment `<%# ... %>`
    Comment,
    /// Escaped tag `<%%`
    EscapedTag,
    /// Generic expression
    Expression,
    /// Identifier
    Identifier,
    /// Literal value
    Literal,
    /// String literal
    String,
    /// Number literal
    Number,
    /// Boolean literal
    Boolean,
    /// Function call
    Function,
    /// Member access expression
    MemberExpression,
    /// Call expression
    CallExpression,
    /// Binary expression
    BinaryExpression,
    /// Unary expression
    UnaryExpression,
    /// Conditional (ternary) expression
    ConditionalExpression,
    /// Array expression
    ArrayExpression,
    /// Object expression
    ObjectExpression,
    /// Arrow function expression
    ArrowFunction,
    /// Error element
    Error,
}

impl ElementType for EjsElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::EjsTokenType> for EjsElementType {
    fn from(token_type: crate::lexer::token_type::EjsTokenType) -> Self {
        use crate::lexer::token_type::EjsTokenType;
        match token_type {
            EjsTokenType::Text => Self::Text,
            EjsTokenType::OpenTag => Self::Code,
            EjsTokenType::OpenTagOutputEscape => Self::OutputEscape,
            EjsTokenType::OpenTagOutputRaw => Self::OutputRaw,
            EjsTokenType::OpenTagComment => Self::Comment,
            EjsTokenType::EscapedOpenTag => Self::EscapedTag,
            EjsTokenType::CloseTag => Self::Code,
            EjsTokenType::CloseTagTrim => Self::Code,
            EjsTokenType::Identifier => Self::Identifier,
            EjsTokenType::String => Self::String,
            EjsTokenType::Number => Self::Number,
            EjsTokenType::Boolean => Self::Boolean,
            EjsTokenType::Whitespace => Self::Text,
            EjsTokenType::Newline => Self::Text,
            EjsTokenType::Comment => Self::Comment,
            EjsTokenType::Eof => Self::Text,
            EjsTokenType::Error => Self::Error,
            _ => Self::Expression,
        }
    }
}
