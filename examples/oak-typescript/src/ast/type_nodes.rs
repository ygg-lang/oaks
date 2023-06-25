use crate::ast::{ClassMember, FunctionParam, LiteralType};
use core::range::Range;

/// Represents a TypeScript type annotation.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TypeAnnotation {
    /// An identifier type.
    Identifier(String),
    /// A predefined type (e.g., `any`, `number`, `string`, `boolean`, `void`, `never`).
    Predefined(String),
    /// A literal type.
    Literal(LiteralType),
    /// An array type.
    Array(Box<TypeAnnotation>),
    /// A tuple type.
    Tuple(Vec<TypeAnnotation>),
    /// A union type.
    Union(Vec<TypeAnnotation>),
    /// An intersection type.
    Intersection(Vec<TypeAnnotation>),
    /// A type reference with optional type arguments.
    Reference {
        /// Name of the referenced type.
        name: String,
        /// Type arguments.
        args: Vec<TypeAnnotation>,
    },
    /// A function type.
    Function {
        /// Type parameters of the function type.
        type_params: Vec<TypeParameter>,
        /// Parameters of the function type.
        args: Vec<FunctionParam>,
        /// Return type of the function type.
        return_type: Box<TypeAnnotation>,
    },
    /// An object type (interface-like).
    Object(Vec<ClassMember>),
    /// A type query (`typeof X`).
    Query(String),
    /// A `keyof` type.
    KeyOf(Box<TypeAnnotation>),
    /// A conditional type (`T extends U ? X : Y`).
    Conditional {
        /// Type being checked.
        check_type: Box<TypeAnnotation>,
        /// Type to extend.
        extends_type: Box<TypeAnnotation>,
        /// Type if the condition is true.
        true_type: Box<TypeAnnotation>,
        /// Type if the condition is false.
        false_type: Box<TypeAnnotation>,
    },
    /// A mapped type (`{ [K in T]: U }`).
    Mapped {
        /// Name of the key variable.
        key_name: String,
        /// Type of the key variable.
        key_type: Box<TypeAnnotation>,
        /// Type of the value.
        value_type: Box<TypeAnnotation>,
        /// Readonly modifier.
        readonly: Option<bool>,
        /// Optional modifier.
        optional: Option<bool>,
    },
    /// A template literal type.
    TemplateLiteral(Vec<TemplateElement>),
    /// An `infer` type.
    Infer(String),
}

/// Represents a type parameter in a generic declaration.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TypeParameter {
    /// Name of the type parameter.
    pub name: String,
    /// Constraint on the type parameter (`extends T`).
    pub constraint: Option<TypeAnnotation>,
    /// Default type of the type parameter (`= T`).
    pub default: Option<TypeAnnotation>,
    /// Source span of the type parameter.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents an element in a template literal type.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TemplateElement {
    /// A literal string element.
    String(String),
    /// A type element.
    Type(Box<TypeAnnotation>),
}
