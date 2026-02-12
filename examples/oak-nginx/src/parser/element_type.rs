use oak_core::{ElementType, UniversalElementRole};

/// Element types for Nginx configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NginxElementType {
    /// The root of the configuration.
    Root,
    /// A configuration directive.
    Directive,
    /// A configuration block.
    Block,
    /// A parameter within a directive.
    Parameter,
    /// A value within a directive or parameter.
    Value,
    /// A comment.
    Comment,
    /// An error element.
    Error,
}

impl NginxElementType {
    /// Returns true if the element type represents a structural element.
    pub fn is_element(&self) -> bool {
        matches!(self, Self::Root | Self::Directive | Self::Block | Self::Parameter | Self::Value | Self::Comment)
    }

    /// Returns true if the element type represents a lexical token.
    pub fn is_token(&self) -> bool {
        !self.is_element()
    }
}

impl ElementType for NginxElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::Directive => UniversalElementRole::Statement,
            Self::Block => UniversalElementRole::Container,
            Self::Parameter => UniversalElementRole::AttributeKey,
            Self::Value => UniversalElementRole::Value,
            Self::Comment => UniversalElementRole::Documentation,
            Self::Error => UniversalElementRole::Error,
        }
    }
}

impl From<crate::lexer::token_type::NginxTokenType> for NginxElementType {
    fn from(token: crate::lexer::token_type::NginxTokenType) -> Self {
        use crate::lexer::token_type::NginxTokenType as T;
        match token {
            T::Root => Self::Root,
            T::Directive => Self::Directive,
            T::Block => Self::Block,
            T::Parameter => Self::Parameter,
            T::Value => Self::Value,
            T::Comment => Self::Comment,
            _ => Self::Error,
        }
    }
}
