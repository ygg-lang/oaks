use super::{Attribute, Block, EnumVariant, EnumsKind, Expr, Function, GenericParam, Identifier, NamePath, Param, Span, Statement, StructureKind, Type, VariantCase};

/// An item in a Valkyrie module
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Item {
    /// A namespace declaration.
    Namespace(Namespace),
    /// A class declaration.
    Class(Class),
    /// A value type structure (immutable, copied on assignment).
    Structure(Class),
    /// A singleton declaration.
    Singleton(Singleton),
    /// A flags (bitflags) declaration.
    Flags(Flags),
    /// An enum declaration.
    Enums(Enums),
    /// A trait declaration.
    Trait(Trait),
    /// A widget declaration.
    Widget(Widget),
    /// A using (import) statement.
    Using(Using),
    /// A micro (small function) declaration.
    Micro(MicroDefinition),
    /// A type function declaration.
    TypeFunction(TypeFunction),
    /// A statement at module level.
    Statement(Statement),
    /// A variant declaration.
    Variant(Variant),
    /// An effect declaration.
    Effect(Effect),
    /// A property declaration (getter or setter).
    Property(Property),
    /// Template text content.
    TemplateText {
        /// The text content.
        content: String,
        /// The source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// Template control structure.
    TemplateControl {
        /// The items within the control structure.
        items: Vec<Item>,
        /// The source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// Template interpolation expression.
    TemplateInterpolation {
        /// The interpolated expression.
        expr: Expr,
        /// The source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
}

/// A namespace declaration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Namespace {
    /// The name path of the namespace.
    pub name: NamePath,
    /// Annotations applied to the namespace.
    pub annotations: Vec<Attribute>,
    /// Items declared within the namespace.
    pub items: Vec<Item>,
    /// The source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}

/// A parent class with optional alias for renamed inheritance.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Parent {
    /// Optional alias for disambiguation (e.g., "primary" in "primary: Parent1").
    pub alias: Option<Identifier>,
    /// Parent class name path.
    pub name: NamePath,
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}

/// A class declaration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Class {
    /// The keyword kind used for this class (class, struct, structure, widget, trait).
    pub kind: StructureKind,
    /// The class name.
    pub name: Identifier,
    /// Generic parameters for the class.
    pub generics: Vec<GenericParam>,
    /// Parent classes or traits this class inherits from.
    pub parents: Vec<Parent>,
    /// Items (fields, methods) declared within the class.
    pub items: Vec<Item>,
    /// Annotations applied to the class.
    pub annotations: Vec<Attribute>,
    /// The source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
    /// Whether this class is abstract (cannot be instantiated directly).
    pub is_abstract: bool,
    /// Whether this class is sealed (restricted inheritance).
    pub is_sealed: bool,
    /// Whether this class is final (cannot be inherited).
    pub is_final: bool,
}

/// A flags (bitflags) declaration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Flags {
    /// The flags name.
    pub name: Identifier,
    /// The flag variants.
    pub variants: Vec<EnumVariant>,
    /// Annotations applied to the flags.
    pub annotations: Vec<Attribute>,
    /// The source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}

/// An enum declaration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Enums {
    /// The keyword kind used for this enum (enums, enum, unity).
    pub kind: EnumsKind,
    /// The enum name.
    pub name: Identifier,
    /// Generic parameters for the enum.
    pub generics: Vec<GenericParam>,
    /// The enum variants.
    pub variants: Vec<EnumVariant>,
    /// Annotations applied to the enum.
    pub annotations: Vec<Attribute>,
    /// The source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}

/// A trait declaration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Trait {
    /// The trait name.
    pub name: Identifier,
    /// Generic parameters for the trait.
    pub generics: Vec<GenericParam>,
    /// Methods declared in the trait.
    pub methods: Vec<Function>,
    /// Associated types declared in the trait.
    pub associated_types: Vec<AssociatedType>,
    /// Annotations applied to the trait.
    pub annotations: Vec<Attribute>,
    /// The source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}

/// An associated type declaration in a trait.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssociatedType {
    /// The associated type name.
    pub name: Identifier,
    /// Type bounds that the associated type must satisfy.
    pub bounds: Vec<Type>,
    /// Default type for the associated type, if any.
    pub default: Option<Type>,
    /// Annotations applied to the associated type.
    pub annotations: Vec<Attribute>,
    /// The source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}

/// A widget declaration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Widget {
    /// The widget name.
    pub name: Identifier,
    /// Generic parameters for the widget.
    pub generics: Vec<GenericParam>,
    /// Items (properties, methods) declared within the widget.
    pub items: Vec<Item>,
    /// Annotations applied to the widget.
    pub annotations: Vec<Attribute>,
    /// The source code span.
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
pub struct Singleton {
    /// The singleton name.
    pub name: Identifier,
    /// Generic parameters for the singleton.
    pub generics: Vec<GenericParam>,
    /// Parent traits this singleton implements.
    pub parents: Vec<Parent>,
    /// Items (fields, methods) declared within the singleton.
    pub items: Vec<Item>,
    /// Annotations applied to the singleton.
    pub annotations: Vec<Attribute>,
    /// The source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}

/// A using (import) statement
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Using {
    /// The path to import.
    pub path: NamePath,
    /// Optional alias for the import.
    pub alias: Option<Identifier>,
    /// The source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}

/// A micro (small function) declaration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MicroDefinition {
    /// The micro name.
    pub name: Identifier,
    /// Generic parameters for the micro.
    pub generics: Vec<GenericParam>,
    /// Parameters for the micro.
    pub params: Vec<Param>,
    /// Return type annotation, if any.
    pub return_type: Option<Type>,
    /// The body of the micro.
    pub body: Block,
    /// Annotations applied to the micro.
    pub annotations: Vec<Attribute>,
    /// The source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
    /// Whether this function is abstract (has no body implementation).
    pub is_abstract: bool,
    /// Whether this function is final (cannot be overridden).
    pub is_final: bool,
}

/// A type function declaration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TypeFunction {
    /// The type function name.
    pub name: Identifier,
    /// Generic parameters for the type function.
    pub generics: Vec<GenericParam>,
    /// Parameters for the type function.
    pub params: Vec<Param>,
    /// Return type annotation, if any.
    pub return_type: Option<Type>,
    /// The body of the type function.
    pub body: Block,
    /// Annotations applied to the type function.
    pub annotations: Vec<Attribute>,
    /// The source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}

/// A variant declaration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Variant {
    /// The variant name.
    pub name: Identifier,
    /// Generic parameters for the variant.
    pub generics: Vec<GenericParam>,
    /// The variant cases.
    pub cases: Vec<VariantCase>,
    /// Annotations applied to the variant.
    pub annotations: Vec<Attribute>,
    /// The source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}

/// An effect declaration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Effect {
    /// The effect name.
    pub name: Identifier,
    /// Operations defined by the effect.
    pub operations: Vec<Function>,
    /// Annotations applied to the effect.
    pub annotations: Vec<Attribute>,
    /// The source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}

/// The kind of a property (getter or setter).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PropertyKind {
    /// A getter property.
    Getter,
    /// A setter property.
    Setter,
}

/// A property declaration (getter or setter).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Property {
    /// The name of the property.
    pub name: Identifier,
    /// Whether this is a getter or setter.
    pub kind: PropertyKind,
    /// Generic parameters for the property.
    pub generics: Vec<GenericParam>,
    /// Annotations on the property.
    pub annotations: Vec<Attribute>,
    /// Parameters for the property (self for getter, self + value for setter).
    pub params: Vec<Param>,
    /// Return type for getter, None for setter.
    pub return_type: Option<Type>,
    /// The body of the property.
    pub body: Block,
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
    /// Whether this property is abstract (has no body implementation).
    ///
    /// Abstract properties are declared without a body in abstract classes
    /// and must be implemented by concrete subclasses.
    pub is_abstract: bool,
    /// Whether this property is final (cannot be overridden).
    pub is_final: bool,
}
