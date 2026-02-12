use super::{common_nodes::RbqAnnotation, expression_nodes::RbqExpr, type_nodes::RbqType};
use crate::{language::RbqLanguage, parser::element_type::RbqElementType};
use oak_core::{Range, tree::RedNode};

/// Represents a struct definition in RBQ.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RbqStruct {
    /// Annotations applied to the struct.
    pub annotations: Vec<RbqAnnotation>,
    /// The name of the struct.
    pub name: String,
    /// Fields contained within the struct.
    pub fields: Vec<RbqField>,
    /// The source range of the struct definition.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl RbqStruct {
    /// Lowers a red node into an `RbqStruct` AST node.
    pub fn lower(red: RedNode<RbqLanguage>, source: &str) -> Self {
        let span = red.span();
        let mut annotations = Vec::new();
        let mut name = String::new();
        let mut fields = Vec::new();

        for child in red.children() {
            match child.kind::<RbqElementType>() {
                RbqElementType::Annotation => {
                    if let Some(node) = child.as_node() {
                        annotations.push(RbqAnnotation::lower(node, source))
                    }
                }
                RbqElementType::Ident if name.is_empty() => name = source[child.span()].trim().to_string(),
                RbqElementType::FieldDef => {
                    if let Some(node) = child.as_node() {
                        fields.push(RbqField::lower(node, source))
                    }
                }
                _ => {}
            }
        }

        Self { annotations, name, fields, span }
    }
}

/// Represents an enum definition in RBQ.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RbqEnum {
    /// Annotations applied to the enum.
    pub annotations: Vec<RbqAnnotation>,
    /// The name of the enum.
    pub name: String,
    /// Variants of the enum.
    pub variants: Vec<String>,
    /// The source range of the enum definition.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl RbqEnum {
    /// Lowers a red node into an `RbqEnum` AST node.
    pub fn lower(red: RedNode<RbqLanguage>, source: &str) -> Self {
        let span = red.span();
        let mut annotations = Vec::new();
        let mut name = String::new();
        let mut variants = Vec::new();

        for child in red.children() {
            match child.kind::<RbqElementType>() {
                RbqElementType::Annotation => {
                    if let Some(node) = child.as_node() {
                        annotations.push(RbqAnnotation::lower(node, source))
                    }
                }
                RbqElementType::Ident if name.is_empty() => name = source[child.span()].trim().to_string(),
                RbqElementType::EnumMember => variants.push(source[child.span()].trim().to_string()),
                _ => {}
            }
        }

        Self { annotations, name, variants, span }
    }
}

/// Represents a union definition in RBQ.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RbqUnion {
    /// Annotations applied to the union.
    pub annotations: Vec<RbqAnnotation>,
    /// The name of the union.
    pub name: String,
    /// Members of the union.
    pub members: Vec<RbqUnionMember>,
    /// The source range of the union definition.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl RbqUnion {
    /// Lowers a red node into an `RbqUnion` AST node.
    pub fn lower(red: RedNode<RbqLanguage>, source: &str) -> Self {
        let span = red.span();
        let mut annotations = Vec::new();
        let mut name = String::new();
        let mut members = Vec::new();

        for child in red.children() {
            match child.kind::<RbqElementType>() {
                RbqElementType::Annotation => {
                    if let Some(node) = child.as_node() {
                        annotations.push(RbqAnnotation::lower(node, source))
                    }
                }
                RbqElementType::Ident if name.is_empty() => name = source[child.span()].trim().to_string(),
                RbqElementType::UnionMember => {
                    if let Some(node) = child.as_node() {
                        members.push(RbqUnionMember::lower(node, source))
                    }
                }
                _ => {}
            }
        }

        Self { annotations, name, members, span }
    }
}

/// Represents a member of a union in RBQ.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RbqUnionMember {
    /// Annotations applied to the union member.
    pub annotations: Vec<RbqAnnotation>,
    /// The name of the union member.
    pub name: String,
    /// Optional payload for the union member.
    pub payload: Option<RbqUnionPayload>,
    /// Optional value associated with the union member.
    pub value: Option<String>,
    /// The source range of the union member definition.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl RbqUnionMember {
    /// Lowers a red node into an `RbqUnionMember` AST node.
    pub fn lower(red: RedNode<RbqLanguage>, source: &str) -> Self {
        let span = red.span();
        let mut annotations = Vec::new();
        let mut name = String::new();
        let mut payload = None;
        let mut value = None;

        for child in red.children() {
            match child.kind::<RbqElementType>() {
                RbqElementType::Annotation => {
                    if let Some(node) = child.as_node() {
                        annotations.push(RbqAnnotation::lower(node, source))
                    }
                }
                RbqElementType::Ident if name.is_empty() => name = source[child.span()].trim().to_string(),
                RbqElementType::TypeDef => {
                    if let Some(node) = child.as_node() {
                        payload = Some(RbqUnionPayload::Struct(node.children().filter_map(|c| if c.kind::<RbqElementType>() == RbqElementType::FieldDef { c.as_node().map(|n| RbqField::lower(n, source)) } else { None }).collect()))
                    }
                }
                RbqElementType::TypeRef => {
                    if let Some(node) = child.as_node() {
                        if payload.is_none() {
                            payload = Some(RbqUnionPayload::Tuple(Vec::new()))
                        }
                        if let Some(RbqUnionPayload::Tuple(types)) = &mut payload {
                            types.push(RbqType::lower(node, source))
                        }
                    }
                }
                RbqElementType::Literal => value = Some(source[child.span()].trim().to_string()),
                _ => {}
            }
        }

        Self { annotations, name, payload, value, span }
    }
}

/// Represents the payload of a union member in RBQ.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RbqUnionPayload {
    /// Tuple payload containing a list of types.
    Tuple(Vec<RbqType>),
    /// Struct payload containing a list of fields.
    Struct(Vec<RbqField>),
}

/// Represents a field definition in RBQ.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RbqField {
    /// Annotations applied to the field.
    pub annotations: Vec<RbqAnnotation>,
    /// The name of the field.
    pub name: String,
    /// The type reference of the field.
    pub type_ref: RbqType,
    /// The default value of the field, if any.
    pub default_value: Option<RbqExpr>,
    /// The source range of the field definition.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl RbqField {
    /// Lowers a red node into an `RbqField` AST node.
    pub fn lower(red: RedNode<RbqLanguage>, source: &str) -> Self {
        let span = red.span();
        let mut annotations = Vec::new();
        let mut name = String::new();
        let mut type_ref = None;
        let mut default_value = None;

        for child in red.children() {
            match child.kind::<RbqElementType>() {
                RbqElementType::Annotation => {
                    if let Some(node) = child.as_node() {
                        annotations.push(RbqAnnotation::lower(node, source))
                    }
                }
                RbqElementType::Ident if name.is_empty() => name = source[child.span()].trim().to_string(),
                RbqElementType::TypeDef | RbqElementType::TypeRef => {
                    if let Some(node) = child.as_node() {
                        type_ref = Some(RbqType::lower(node, source))
                    }
                }
                _ => {
                    if let Some(node) = child.as_node() {
                        if node.kind::<RbqElementType>() == RbqElementType::Expression {
                            default_value = Some(RbqExpr::lower(node, source))
                        }
                    }
                }
            }
        }

        Self { annotations, name, type_ref: type_ref.unwrap_or(RbqType::Named { path: "any".to_string(), generic_args: Vec::new(), is_physical_ptr: false, is_optional: false, span: span.clone() }), default_value, span }
    }
}
