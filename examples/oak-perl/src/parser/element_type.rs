use oak_core::{ElementType, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PerlElementType {
    Root,
    Program,
    Statement,
    Expression,
    Block,
    SubroutineDeclaration,
    PackageDeclaration,
    UseStatement,
    VariableDeclaration,
    Assignment,
    FunctionCall,
    MethodCall,
    ArrayAccess,
    HashAccess,
    Reference,
    Dereference,
    ConditionalExpression,
    LoopStatement,
    IfStatement,
    UnlessStatement,
    WhileStatement,
    UntilStatement,
    ForStatement,
    ForeachStatement,
    DoStatement,
    EvalStatement,
    RegexMatch,
    RegexSubstitution,
    RegexTransliteration,
    Error,
}

impl PerlElementType {
    pub fn is_token(&self) -> bool {
        false
    }

    pub fn is_element(&self) -> bool {
        true
    }
}

impl ElementType for PerlElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        use UniversalElementRole::*;
        match self {
            Self::Root | Self::Program => Root,
            Self::Statement | Self::UseStatement | Self::LoopStatement | Self::IfStatement | Self::UnlessStatement | Self::WhileStatement | Self::UntilStatement | Self::ForStatement | Self::ForeachStatement | Self::DoStatement | Self::EvalStatement => {
                Statement
            }
            Self::Expression
            | Self::Assignment
            | Self::FunctionCall
            | Self::MethodCall
            | Self::ArrayAccess
            | Self::HashAccess
            | Self::Reference
            | Self::Dereference
            | Self::ConditionalExpression
            | Self::RegexMatch
            | Self::RegexSubstitution
            | Self::RegexTransliteration => Expression,
            Self::Block => Statement,
            Self::SubroutineDeclaration | Self::PackageDeclaration | Self::VariableDeclaration => Definition,
            Self::Error => Error,
        }
    }
}

impl From<crate::lexer::token_type::PerlTokenType> for PerlElementType {
    fn from(token: crate::lexer::token_type::PerlTokenType) -> Self {
        use crate::lexer::token_type::PerlTokenType;
        match token {
            PerlTokenType::InternalProgram => PerlElementType::Program,
            PerlTokenType::InternalStatement => PerlElementType::Statement,
            PerlTokenType::InternalExpression => PerlElementType::Expression,
            PerlTokenType::InternalBlock => PerlElementType::Block,
            _ => PerlElementType::Error,
        }
    }
}
