use oak_core::{ElementType, Parser, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum NginxElementType {
    Root,
    Directive,
    Block,
    Parameter,
    Value,
    Comment,
    Error,
}

impl NginxElementType {
    pub fn is_element(&self) -> bool {
        matches!(self, Self::Root | Self::Directive | Self::Block | Self::Parameter | Self::Value | Self::Comment)
    }

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
