use crate::lexer::token_type::JinjaTokenType;
use oak_core::{ElementType, UniversalElementRole};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum JinjaElementType {
    Root,
    HtmlContent,
    Comment,
    Variable,
    Tag,
    Block,
    IfStatement,
    ForStatement,
    MacroDefinition,
    CallBlock,
    FilterBlock,
    SetStatement,
    WithBlock,
    AutoescapeBlock,
    ImportStatement,
    FromStatement,
    Expression,
    Identifier,
    Literal,
    UnaryExpression,
    BinaryExpression,
    MemberExpression,
    CallExpression,
    FilterExpression,
    TestExpression,
    Error,
}

impl ElementType for JinjaElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::Identifier => UniversalElementRole::Reference,
            Self::Literal => UniversalElementRole::Value,
            Self::Expression | Self::UnaryExpression | Self::BinaryExpression | Self::MemberExpression | Self::CallExpression | Self::FilterExpression | Self::TestExpression => UniversalElementRole::Expression,
            Self::IfStatement | Self::ForStatement | Self::SetStatement | Self::ImportStatement | Self::FromStatement => UniversalElementRole::Statement,
            Self::MacroDefinition => UniversalElementRole::Definition,
            Self::Block | Self::CallBlock | Self::FilterBlock | Self::WithBlock | Self::AutoescapeBlock => UniversalElementRole::Container,
            Self::Comment => UniversalElementRole::Documentation,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<JinjaTokenType> for JinjaElementType {
    fn from(token: JinjaTokenType) -> Self {
        match token {
            JinjaTokenType::Error => Self::Error,
            _ => Self::Error, // Default fallback for tokens that don't map directly to element types
        }
    }
}
