use oak_core::{ElementType, UniversalElementRole};

/// Element types for Dejavu language syntax tree nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DejavuElementType {
    /// Root node of the syntax tree.
    Root,
    /// End of file marker.
    Eof,
    /// Whitespace token.
    Whitespace,
    /// Line comment token.
    LineComment,
    /// Block comment token.
    BlockComment,
    /// Error node.
    Error,

    // Nodes
    /// Attribute node.
    Attribute,
    /// Name path node.
    NamePath,
    /// Type node.
    Type,
    /// Namespace node.
    Namespace,
    /// Class node.
    Class,
    /// Parameter list node.
    ParameterList,
    /// Block expression node.
    BlockExpression,
    /// Identifier expression node.
    IdentifierExpression,
    /// Path expression node.
    PathExpression,

    // Statements
    /// Statement node.
    Statement,
    /// Let statement node.
    LetStatement,
    /// Expression statement node.
    ExprStatement,

    // Expressions
    /// Expression node.
    Expression,
    /// Call expression node.
    CallExpression,
    /// Parameter node.
    Param,
    /// Return expression node.
    ReturnExpression,
    /// Literal expression node.
    LiteralExpression,
    /// Boolean literal node.
    BooleanLiteral,
    /// Binary expression node.
    BinaryExpression,
    /// Unary expression node.
    UnaryExpression,
    /// Parenthesized expression node.
    ParenthesizedExpression,
    /// Index expression node.
    IndexExpression,
    /// Field expression node.
    FieldExpression,
    /// If expression node.
    IfExpression,
    /// Match expression node.
    MatchExpression,
    /// Loop expression node.
    LoopExpression,
    /// Break expression node.
    BreakExpression,
    /// Continue expression node.
    ContinueExpression,
    /// Yield expression node.
    YieldExpression,
    /// Raise expression node.
    RaiseExpression,
    /// Catch expression node.
    CatchExpression,
    /// Resume expression node.
    ResumeExpression,
    /// Apply block node.
    ApplyBlock,
    /// Object expression node.
    ObjectExpression,

    // Definitions
    /// Micro definition node.
    Micro,
    /// Mezzo definition node.
    Mezzo,
    /// Macro definition node.
    Macro,
    /// Struct definition node.
    Struct,
    /// Enum definition node.
    Enum,
    /// Enums definition node.
    Enums,
    /// Trait definition node.
    Trait,
    /// Impl definition node.
    Impl,
    /// Field definition node.
    Field,
    /// Method definition node.
    Method,
    /// Variant definition node.
    Variant,
    /// Flags definition node.
    Flags,
    /// Widget definition node.
    Widget,
    /// Effect definition node.
    EffectDefinition,
    /// Using statement node.
    UsingStatement,

    // Template
    /// Template text node.
    TemplateText,
    /// Template control node.
    TemplateControl,
    /// Interpolation node.
    Interpolation,
    /// Template comment node.
    TemplateComment,

    // Others
    /// Pattern node.
    Pattern,
    /// Argument list node.
    ArgList,
    /// Generic parameter list node.
    GenericParameterList,
    /// Generic argument list node.
    GenericArgumentList,
    /// Match arm node.
    MatchArm,
    /// Parameter node.
    Parameter,
    /// Anonymous class node.
    AnonymousClass,
}

impl ElementType for DejavuElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::DejavuTokenType> for DejavuElementType {
    fn from(token: crate::lexer::token_type::DejavuTokenType) -> Self {
        match token {
            crate::lexer::token_type::DejavuTokenType::Eof => Self::Eof,
            crate::lexer::token_type::DejavuTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::DejavuTokenType::Error => Self::Error,
            _ => Self::Error,
        }
    }
}
