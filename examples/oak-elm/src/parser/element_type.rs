use oak_core::{ElementType, UniversalElementRole};

/// Element types for Elm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ElmElementType {
    /// The root node of the AST.
    Root,
    /// A module declaration.
    Module,
    /// An import statement.
    Import,
    /// A custom type declaration.
    TypeDeclaration,
    /// A type alias declaration.
    TypeAlias,
    /// A function declaration.
    FunctionDeclaration,
    /// A generic expression.
    Expression,
    /// A literal value.
    Literal,
    /// An identifier.
    Identifier,
    /// A binary expression.
    BinaryExpression,
    /// A unary expression.
    UnaryExpression,
    /// An `if` expression.
    IfExpression,
    /// A `case` expression.
    CaseExpression,
    /// A `let` expression.
    LetExpression,
    /// A tuple expression.
    TupleExpression,
    /// A list expression.
    ListExpression,
    /// A record expression.
    RecordExpression,
    /// A field access expression.
    FieldExpression,
    /// A lambda expression.
    LambdaExpression,
    /// A type signature.
    TypeSignature,
    /// A value declaration.
    ValueDeclaration,
    /// A pattern.
    Pattern,
    /// An error element.
    Error,
}

impl ElementType for ElmElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::Module => UniversalElementRole::Definition,
            Self::Import => UniversalElementRole::Statement,
            Self::FunctionDeclaration => UniversalElementRole::Definition,
            Self::Expression => UniversalElementRole::Expression,
            Self::Identifier => UniversalElementRole::Name,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::ElmTokenType> for ElmElementType {
    fn from(token: crate::lexer::token_type::ElmTokenType) -> Self {
        match token {
            crate::lexer::token_type::ElmTokenType::Root => Self::Root,
            crate::lexer::token_type::ElmTokenType::Identifier => Self::Identifier,
            _ => unsafe { std::mem::transmute(token) },
        }
    }
}
