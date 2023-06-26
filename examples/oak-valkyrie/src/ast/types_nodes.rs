use super::{Expr, Identifier, NamePath, Span};

/// A type expression
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    /// A named type (e.g., `String`, `i32`).
    Named {
        /// The type name path.
        path: NamePath,
        /// The source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// A generic type parameter.
    Generic {
        /// The generic parameter name.
        name: Identifier,
        /// The source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// A tuple type.
    Tuple {
        /// The element types.
        elements: Vec<Type>,
        /// The source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// A function type.
    Function {
        /// The parameter types.
        params: Vec<Type>,
        /// The return type.
        return_type: Box<Type>,
        /// The source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// An optional type.
    Optional {
        /// The inner type.
        inner: Box<Type>,
        /// The source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// An associated type projection (e.g., `Self::Item`, `T::Output`).
    AssociatedType {
        /// The base type (e.g., `Self` or a type parameter name).
        base: Identifier,
        /// The associated type name.
        name: Identifier,
        /// The source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// A qualified associated type (e.g., `<T as Trait>::Item`).
    QualifiedAssociatedType {
        /// The type being projected from.
        ty: Box<Type>,
        /// The trait providing the associated type.
        trait_path: NamePath,
        /// The associated type name.
        name: Identifier,
        /// The source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
}

/// A generic parameter
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GenericParam {
    /// The generic parameter name.
    pub name: Identifier,
    /// Type constraints (bounds) for the generic parameter.
    pub constraints: Vec<Type>,
    /// Default type for the generic parameter.
    pub default: Option<Type>,
    /// The source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}

/// A function parameter
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Param {
    /// The parameter name.
    pub name: Identifier,
    /// Optional type annotation.
    pub ty: Option<Type>,
    /// Optional default value expression.
    pub default: Option<Expr>,
    /// The source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}
