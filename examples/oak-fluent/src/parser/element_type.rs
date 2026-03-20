/// Fluent element types.
use oak_core::ElementRole;

/// Fluent element types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FluentElementType {
    /// Root element.
    Root,
    /// Message element.
    Message,
    /// Attribute element.
    Attribute,
    /// Variant element.
    Variant,
    /// Pattern element.
    Pattern,
    /// Text element.
    Text,
    /// Variable reference element.
    VariableReference,
    /// Message reference element.
    MessageReference,
    /// Select expression element.
    SelectExpression,
    /// Expression element.
    Expression,
}

impl oak_core::ElementType for FluentElementType {
    /// The associated role type.
    type Role = FluentElementRole;

    /// Returns the role of the element.
    fn role(&self) -> Self::Role {
        match self {
            Self::Root => FluentElementRole::Root,
            Self::Message => FluentElementRole::Message,
            Self::Attribute => FluentElementRole::Attribute,
            Self::Variant => FluentElementRole::Variant,
            Self::Pattern => FluentElementRole::Pattern,
            Self::Text => FluentElementRole::Text,
            Self::VariableReference => FluentElementRole::Variable,
            Self::MessageReference => FluentElementRole::Reference,
            Self::SelectExpression => FluentElementRole::Expression,
            Self::Expression => FluentElementRole::Expression,
        }
    }
}

/// Fluent element roles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FluentElementRole {
    /// Root role.
    Root,
    /// Message role.
    Message,
    /// Attribute role.
    Attribute,
    /// Variant role.
    Variant,
    /// Pattern role.
    Pattern,
    /// Text role.
    Text,
    /// Variable role.
    Variable,
    /// Reference role.
    Reference,
    /// Expression role.
    Expression,
}

impl ElementRole for FluentElementRole {
    /// Maps this role to a universal, language-agnostic role.
    fn universal(&self) -> oak_core::language::UniversalElementRole {
        match self {
            Self::Root => oak_core::language::UniversalElementRole::Root,
            Self::Message => oak_core::language::UniversalElementRole::Definition,
            Self::Attribute => oak_core::language::UniversalElementRole::Attribute,
            Self::Variant => oak_core::language::UniversalElementRole::Detail,
            Self::Pattern => oak_core::language::UniversalElementRole::Container,
            Self::Text => oak_core::language::UniversalElementRole::Value,
            Self::Variable => oak_core::language::UniversalElementRole::Reference,
            Self::Reference => oak_core::language::UniversalElementRole::Reference,
            Self::Expression => oak_core::language::UniversalElementRole::Expression,
        }
    }

    /// Returns a specific name for this role, used for granular highlighting.
    fn name(&self) -> &str {
        match self {
            Self::Root => "source",
            Self::Message => "entity.name.function",
            Self::Attribute => "entity.other.attribute-name",
            Self::Variant => "meta.detail",
            Self::Pattern => "meta.block",
            Self::Text => "constant",
            Self::Variable => "variable.other.usage",
            Self::Reference => "variable.other.usage",
            Self::Expression => "meta.expression",
        }
    }
}

impl From<super::super::lexer::token_type::FluentTokenKind> for FluentElementType {
    fn from(token_type: super::super::lexer::token_type::FluentTokenKind) -> Self {
        match token_type {
            super::super::lexer::token_type::FluentTokenKind::Identifier => Self::Text,
            super::super::lexer::token_type::FluentTokenKind::StringLiteral => Self::Text,
            super::super::lexer::token_type::FluentTokenKind::NumberLiteral => Self::Text,
            _ => Self::Text, // Default to Text for other token types
        }
    }
}
