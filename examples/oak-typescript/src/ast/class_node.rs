use crate::ast::{Decorator, Expression, FunctionParam, Statement, TypeAnnotation, TypeParameter};
use core::range::Range;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Visibility {
    Public,
    Private,
    Protected,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ClassMember {
    Property {
        decorators: Vec<Decorator>,
        name: String,
        ty: Option<TypeAnnotation>,
        initializer: Option<Expression>,
        visibility: Option<Visibility>,
        is_static: bool,
        is_readonly: bool,
        is_abstract: bool,
        is_optional: bool,
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    Method {
        decorators: Vec<Decorator>,
        name: String,
        type_params: Vec<TypeParameter>,
        params: Vec<FunctionParam>,
        return_type: Option<TypeAnnotation>,
        body: Vec<Statement>,
        visibility: Option<Visibility>,
        is_static: bool,
        is_abstract: bool,
        is_getter: bool,
        is_setter: bool,
        is_optional: bool,
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
}
