use oak_core::{ElementType, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SqlElementType {
    Root,
    Identifier,
    Expression,
    ErrorNode,
    SelectStatement,
    InsertStatement,
    UpdateStatement,
    DeleteStatement,
    CreateStatement,
    DropStatement,
    AlterStatement,
    JoinClause,
    GroupByClause,
    HavingClause,
    OrderByClause,
    LimitClause,
    TableName,
    ColumnName,
    SelectItem,
    Alias,
    ColumnDefinition,
    ValueList,
    Assignment,
}

impl ElementType for SqlElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> UniversalElementRole {
        use UniversalElementRole::*;
        match self {
            Self::Root => Root,
            Self::Identifier | Self::TableName | Self::ColumnName | Self::Alias => Name,
            Self::Expression => Expression,
            Self::ErrorNode => Error,
            Self::SelectStatement | Self::InsertStatement | Self::UpdateStatement | Self::DeleteStatement | Self::CreateStatement | Self::DropStatement | Self::AlterStatement => Statement,
            Self::JoinClause | Self::GroupByClause | Self::HavingClause | Self::OrderByClause | Self::LimitClause | Self::SelectItem | Self::ColumnDefinition | Self::ValueList | Self::Assignment => Statement,
        }
    }
}

impl From<crate::lexer::token_type::SqlTokenType> for SqlElementType {
    fn from(token: crate::lexer::token_type::SqlTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
