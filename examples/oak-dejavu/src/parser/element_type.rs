use crate::lexer::token_type::DejavuSyntaxKind;
use oak_core::{ElementType, UniversalElementRole};

pub type DejavuElementType = DejavuSyntaxKind;

impl ElementType for DejavuSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::SourceFile | Self::DejavuRoot => UniversalElementRole::Root,
            Self::Namespace => UniversalElementRole::Container,
            Self::NamePath => UniversalElementRole::Expression,
            Self::UsingStatement => UniversalElementRole::Statement,
            Self::Class => UniversalElementRole::Definition,
            Self::Widget => UniversalElementRole::Definition,
            Self::EffectDefinition => UniversalElementRole::Definition,
            Self::Micro => UniversalElementRole::Definition,
            Self::Mezzo => UniversalElementRole::Definition,
            Self::ParameterList => UniversalElementRole::Container,
            Self::Parameter => UniversalElementRole::Binding,
            Self::BlockExpression => UniversalElementRole::Expression,
            Self::LetStatement => UniversalElementRole::Statement,
            Self::ExpressionStatement => UniversalElementRole::Statement,
            Self::IdentifierExpression | Self::PathExpression | Self::LiteralExpression => UniversalElementRole::Expression,
            Self::BooleanLiteral => UniversalElementRole::Expression,
            Self::AnonymousClass => UniversalElementRole::Expression,
            Self::ApplyBlock | Self::ObjectExpression => UniversalElementRole::Expression,
            Self::ParenthesizedExpression => UniversalElementRole::Expression,
            Self::UnaryExpression => UniversalElementRole::Expression,
            Self::BinaryExpression => UniversalElementRole::Expression,
            Self::CallExpression => UniversalElementRole::Call,
            Self::FieldExpression => UniversalElementRole::Expression,
            Self::IndexExpression => UniversalElementRole::Expression,
            Self::IfExpression => UniversalElementRole::Expression,
            Self::MatchExpression => UniversalElementRole::Expression,
            Self::MatchArm => UniversalElementRole::Container,
            Self::LoopExpression => UniversalElementRole::Expression,
            Self::ReturnExpression => UniversalElementRole::Expression,
            Self::BreakExpression => UniversalElementRole::Expression,
            Self::ContinueExpression => UniversalElementRole::Expression,
            Self::YieldExpression => UniversalElementRole::Expression,
            Self::RaiseExpression => UniversalElementRole::Expression,
            Self::CatchExpression => UniversalElementRole::Expression,
            Self::ResumeExpression => UniversalElementRole::Expression,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }

    fn is_root(&self) -> bool {
        matches!(self, Self::SourceFile | Self::DejavuRoot)
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }
}
