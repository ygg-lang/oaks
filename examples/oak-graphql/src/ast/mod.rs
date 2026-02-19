#![doc = include_str!("readme.md")]
use oak_core::Range;

type SourceSpan = Range<usize>;

/// GraphQL root node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GraphQLRoot {
    /// The document node.
    pub document: Document,
}

impl GraphQLRoot {
    /// Creates a new GraphQL root.
    pub fn new(document: Document) -> Self {
        Self { document }
    }
}

/// GraphQL document.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Document {
    /// List of definitions.
    pub definitions: Vec<Definition>,
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: SourceSpan,
}

impl Document {
    /// Creates a new document with the given span.
    pub fn new(span: SourceSpan) -> Self {
        Self { definitions: Vec::new(), span }
    }

    /// Adds a definition to the document.
    pub fn with_definition(mut self, definition: Definition) -> Self {
        self.definitions.push(definition);
        self
    }
}

/// GraphQL definition.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Definition {
    /// An operation definition.
    Operation(OperationDefinition),
    /// A fragment definition.
    Fragment(FragmentDefinition),
    /// A schema definition.
    Schema(SchemaDefinition),
    /// A type definition.
    Type(TypeDefinition),
}

/// GraphQL operation definition.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OperationDefinition {
    /// The type of operation.
    pub operation_type: OperationType,
    /// Optional name of the operation.
    pub name: Option<String>,
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: SourceSpan,
}

impl OperationDefinition {
    /// Creates a new operation definition.
    pub fn new(operation_type: OperationType, span: SourceSpan) -> Self {
        Self { operation_type, name: None, span }
    }

    /// Sets the name of the operation.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

/// GraphQL operation type.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OperationType {
    /// A query operation.
    Query,
    /// A mutation operation.
    Mutation,
    /// A subscription operation.
    Subscription,
}

/// GraphQL fragment definition.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FragmentDefinition {
    /// Name of the fragment.
    pub name: String,
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: SourceSpan,
}

impl FragmentDefinition {
    /// Creates a new fragment definition.
    pub fn new(name: impl Into<String>, span: SourceSpan) -> Self {
        Self { name: name.into(), span }
    }
}

/// GraphQL schema definition.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SchemaDefinition {
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: SourceSpan,
}

impl SchemaDefinition {
    /// Creates a new schema definition.
    pub fn new(span: SourceSpan) -> Self {
        Self { span }
    }
}

/// GraphQL type definition.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TypeDefinition {
    /// Name of the type.
    pub name: String,
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: SourceSpan,
}

impl TypeDefinition {
    /// Creates a new type definition.
    pub fn new(name: impl Into<String>, span: SourceSpan) -> Self {
        Self { name: name.into(), span }
    }
}
