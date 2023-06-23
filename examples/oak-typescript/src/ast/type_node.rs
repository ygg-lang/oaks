use crate::ast::{ClassMember, FunctionParam, LiteralType};
use core::range::Range;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TypeAnnotation {
    Identifier(String),
    Predefined(String), // any, number, string, etc.
    Literal(LiteralType),
    Array(Box<TypeAnnotation>),
    Tuple(Vec<TypeAnnotation>),
    Union(Vec<TypeAnnotation>),
    Intersection(Vec<TypeAnnotation>),
    Reference { name: String, args: Vec<TypeAnnotation> },
    Function { params: Vec<TypeParameter>, args: Vec<FunctionParam>, return_type: Box<TypeAnnotation> },
    Object(Vec<ClassMember>),
    Query(String), // typeof X
    KeyOf(Box<TypeAnnotation>),
    Conditional { check_type: Box<TypeAnnotation>, extends_type: Box<TypeAnnotation>, true_type: Box<TypeAnnotation>, false_type: Box<TypeAnnotation> },
    Mapped { key_name: String, key_type: Box<TypeAnnotation>, value_type: Box<TypeAnnotation>, readonly: Option<bool>, optional: Option<bool> },
    TemplateLiteral(Vec<TemplateElement>),
    Infer(String),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TypeParameter {
    pub name: String,
    pub constraint: Option<TypeAnnotation>,
    pub default: Option<TypeAnnotation>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TemplateElement {
    String(String),
    Type(Box<TypeAnnotation>),
}
