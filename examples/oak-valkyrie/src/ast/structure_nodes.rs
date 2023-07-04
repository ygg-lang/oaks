use crate::ast::{Attribute, Block, GenericParam, Identifier, Param, Parent, Span, TermExpression, TypeExpression};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StructureDeclaration {
    /// The class name.
    pub name: Identifier,
    /// Generic parameters for the class.
    pub generics: Vec<GenericParam>,
    /// Parent classes or traits this class inherits from.
    pub parents: Vec<Parent>,
    /// Items (fields, methods) declared within the class.
    pub fields: Vec<FieldDeclaration>,
    /// Annotations applied to the class.
    pub annotations: Vec<Attribute>,
    /// The source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}

/// A class declaration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClassDeclaration {
    /// Annotations applied to the class.
    pub annotations: Vec<Attribute>,
    /// The class name.
    pub name: Identifier,
    /// Generic parameters for the class.
    pub generics: Vec<GenericParam>,
    /// Parent classes or traits this class inherits from.
    pub parents: Vec<Parent>,
    /// Fields declared within the class.
    pub fields: Vec<FieldDeclaration>,
    /// Methods declared within the class.
    pub methods: Vec<MethodDeclaration>,
    /// The source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}

/// A lambda expression
///
/// ```v
/// let add = class { x: 10, y: 10 }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AnonymousClass {
    /// Parent traits or classes to implement/extend.
    pub parents: Vec<String>,
    /// Fields declared within the class.
    pub fields: Vec<FieldDeclaration>,
    /// Methods declared within the class.
    pub methods: Vec<MethodDeclaration>,
    /// Variables captured from the enclosing scope.
    pub captures: Vec<Identifier>,
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}

/// A singleton declaration.
///
/// Singletons are classes that have exactly one instance globally.
/// They are useful for managing global state, configuration, or resources.
///
/// # Example
///
/// ```v
/// singleton GlobalConfig {
///     host: String = "localhost"
///     port: i32 = 8080
///
///     micro get_url(self) -> String {
///         f"{self.host}:{self.port}"
///     }
/// }
/// ```
///
/// # Semantics
///
/// - A singleton has exactly one global instance
/// - The instance is lazily initialized on first access
/// - Singleton members are accessed through the singleton name directly
/// - Singletons cannot be instantiated with constructors
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SingletonDeclaration {
    /// The singleton name.
    pub name: Identifier,
    /// Generic parameters for the singleton.
    pub generics: Vec<GenericParam>,
    /// Parent traits this singleton implements.
    pub parents: Vec<Parent>,
    /// Fields declared within the class.
    pub fields: Vec<FieldDeclaration>,
    /// Methods declared within the class.
    pub methods: Vec<MethodDeclaration>,
    /// Annotations applied to the singleton.
    pub annotations: Vec<Attribute>,
    /// The source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}

/// A field in a class or struct
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FieldDeclaration {
    /// The field name.
    pub name: Identifier,
    /// The field type.
    pub ty: TypeExpression,
    /// Optional default value expression.
    pub default: Option<TermExpression>,
    /// Annotations applied to the field.
    pub annotations: Vec<Attribute>,
    /// The source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}

/// A function definition
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MethodDeclaration {
    /// The function name.
    pub name: Identifier,
    /// Generic parameters for the function.
    pub generics: Vec<GenericParam>,
    /// The function parameters.
    pub params: Vec<Param>,
    /// Optional return type annotation.
    pub return_type: Option<TypeExpression>,
    /// The optional function body.
    pub body: Option<Block>,
    /// Annotations applied to the function.
    pub annotations: Vec<Attribute>,
    /// The source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}
