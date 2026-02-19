use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};

/// Java element type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum JavaElementType {
    /// Token.
    Token(crate::lexer::token_type::JavaTokenType),
    /// Root node.
    Root,
    /// Identifier.
    Identifier,
    /// Literal expression.
    LiteralExpression,
    /// Parenthesized expression.
    ParenthesizedExpression,
    /// Array creation expression.
    ArrayCreation,
    /// Method call expression.
    MethodCall,
    /// Unary expression.
    UnaryExpression,
    /// Binary expression.
    BinaryExpression,
    /// Assignment expression.
    AssignmentExpression,
    /// Cast expression.
    CastExpression,
    /// Postfix expression.
    PostfixExpression,
    /// Ternary expression.
    TernaryExpression,
    /// Member select expression.
    MemberSelect,
    /// Array access expression.
    ArrayAccess,
    /// New expression.
    NewExpression,
    /// Variable declaration.
    VariableDeclaration,
    /// Expression statement.
    ExpressionStatement,
    /// If statement.
    IfStatement,
    /// While statement.
    WhileStatement,
    /// Do-while statement.
    DoWhileStatement,
    /// For statement.
    ForStatement,
    /// Switch statement.
    SwitchStatement,
    /// Return statement.
    ReturnStatement,
    /// Break statement.
    Break,
    /// Continue statement.
    Continue,
    /// Parameter.
    Parameter,
    /// Catch clause.
    CatchClause,
    /// Try statement.
    TryStatement,
    /// Throw statement.
    ThrowStatement,
    /// Throws clause.
    ThrowsClause,
    /// Package declaration.
    Package,
    /// Import declaration.
    Import,
    /// Class declaration.
    ClassDeclaration,
    /// Interface declaration.
    InterfaceDeclaration,
    /// Enum declaration.
    EnumDeclaration,
    /// Struct declaration.
    StructDeclaration,
    /// Record declaration.
    RecordDeclaration,
    /// Method declaration.
    MethodDeclaration,
    /// Constructor declaration.
    ConstructorDeclaration,
    /// Field declaration.
    FieldDeclaration,
    /// Annotation.
    Annotation,
    /// Switch case.
    SwitchCase,
    /// Default case.
    DefaultCase,
    /// Block statement.
    BlockStatement,
    /// Compilation unit.
    CompilationUnit,
    /// Error element.
    Error,
    /// Anonymous class.
    AnonymousClass,
    /// Type parameter.
    TypeParameter,
    /// Type parameter constraint.
    TypeParameterConstraint,
    /// Generic type.
    GenericType,
    /// Lambda expression.
    LambdaExpression,
    /// Annotation type declaration.
    AnnotationTypeDeclaration,
    /// Annotation element.
    AnnotationElement,
    /// Enum constant.
    EnumConstant,
}

impl ElementType for JavaElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        use UniversalElementRole::*;
        match self {
            Self::Token(token) => match token.role() {
                UniversalTokenRole::Keyword => None,
                UniversalTokenRole::Name => Name,
                UniversalTokenRole::Literal => Value,
                UniversalTokenRole::Operator => Expression,
                UniversalTokenRole::Punctuation => None,
                UniversalTokenRole::Comment => Documentation,
                UniversalTokenRole::Whitespace => None,
                UniversalTokenRole::Error => Error,
                UniversalTokenRole::None => None,
                UniversalTokenRole::Eof => None,
                UniversalTokenRole::Escape => Value,
            },
            Self::Root | Self::CompilationUnit => Root,
            Self::Identifier => Name,
            Self::LiteralExpression => Expression,
            Self::ParenthesizedExpression
            | Self::ArrayCreation
            | Self::MethodCall
            | Self::UnaryExpression
            | Self::BinaryExpression
            | Self::AssignmentExpression
            | Self::CastExpression
            | Self::PostfixExpression
            | Self::TernaryExpression
            | Self::MemberSelect
            | Self::ArrayAccess
            | Self::NewExpression => Expression,
            Self::VariableDeclaration
            | Self::ExpressionStatement
            | Self::IfStatement
            | Self::WhileStatement
            | Self::DoWhileStatement
            | Self::ForStatement
            | Self::SwitchStatement
            | Self::ReturnStatement
            | Self::Break
            | Self::Continue
            | Self::TryStatement
            | Self::ThrowStatement => Statement,
            Self::ClassDeclaration | Self::InterfaceDeclaration | Self::EnumDeclaration | Self::StructDeclaration | Self::RecordDeclaration | Self::MethodDeclaration | Self::FieldDeclaration | Self::ConstructorDeclaration | Self::AnonymousClass => {
                Definition
            }
            Self::Parameter | Self::CatchClause | Self::SwitchCase | Self::DefaultCase | Self::ThrowsClause | Self::Annotation | Self::TypeParameter | Self::TypeParameterConstraint | Self::AnnotationElement | Self::EnumConstant => Detail,
            Self::Package | Self::Import => Statement,
            Self::BlockStatement => Container,
            Self::LambdaExpression | Self::GenericType => Expression,
            Self::AnnotationTypeDeclaration => Definition,
            Self::Error => Error,
        }
    }
}

impl From<crate::lexer::token_type::JavaTokenType> for JavaElementType {
    fn from(token: crate::lexer::token_type::JavaTokenType) -> Self {
        Self::Token(token)
    }
}
