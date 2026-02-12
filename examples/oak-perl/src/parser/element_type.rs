use oak_core::{ElementType, UniversalElementRole};

/// Element type for the Perl language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PerlElementType {
    /// Root node.
    Root,
    /// Program.
    Program,
    /// Statement.
    Statement,
    /// Expression.
    Expression,
    /// Block.
    Block,
    /// Subroutine declaration.
    SubroutineDeclaration,
    /// Package declaration.
    PackageDeclaration,
    /// Use statement.
    UseStatement,
    /// Variable declaration.
    VariableDeclaration,
    /// Assignment.
    Assignment,
    /// Function call.
    FunctionCall,
    /// Method call.
    MethodCall,
    /// Array access.
    ArrayAccess,
    /// Hash access.
    HashAccess,
    /// Reference.
    Reference,
    /// Dereference.
    Dereference,
    /// Conditional expression.
    ConditionalExpression,
    /// Loop statement.
    LoopStatement,
    /// If statement.
    IfStatement,
    /// Unless statement.
    UnlessStatement,
    /// While statement.
    WhileStatement,
    /// Until statement.
    UntilStatement,
    /// For statement.
    ForStatement,
    /// Foreach statement.
    ForeachStatement,
    /// Do statement.
    DoStatement,
    /// Eval statement.
    EvalStatement,
    /// Regex match.
    RegexMatch,
    /// Regex substitution.
    RegexSubstitution,
    /// Regex transliteration.
    RegexTransliteration,
    /// Error.
    Error,
}

impl PerlElementType {
    /// Returns `true` if this element type is a token.
    pub fn is_token(&self) -> bool {
        false
    }

    /// Returns `true` if this element type is an element.
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
