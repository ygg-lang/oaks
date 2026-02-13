#![doc = include_str!("readme.md")]
use crate::{language::RbqLanguage, lexer::token_type::RbqTokenType};
use core::range::Range;
use oak_core::{
    TokenType,
    tree::{RedNode, RedTree},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RbqRoot {
    pub items: Vec<RbqItem>,
}

impl RbqRoot {
    pub fn lower(red: RedNode<RbqLanguage>, source: &str) -> Self {
        let mut items = Vec::new();
        let mut pending_annotations = Vec::new();

        for child in red.children() {
            match child {
                RedTree::Node(node) => match node.kind::<RbqTokenType>() {
                    RbqTokenType::Annotation => pending_annotations.push(RbqAnnotation::lower(node, source)),
                    RbqTokenType::NamespaceDef => {
                        let ns = RbqNamespace::lower(node, source);
                        // If annotations were before a namespace, they should probably be ignored or handled if supported.
                        // For now we clear them to avoid applying them to the next item.
                        pending_annotations.clear();
                        items.push(RbqItem::Namespace(ns))
                    }
                    RbqTokenType::ImportDef => {
                        pending_annotations.clear();
                        items.push(RbqItem::Import(RbqImport::lower(node, source)))
                    }
                    RbqTokenType::StructDef | RbqTokenType::ClassDef => {
                        let mut s = RbqStruct::lower(node, source);
                        s.annotations.extend(pending_annotations.drain(..));
                        items.push(RbqItem::Struct(s))
                    }
                    RbqTokenType::EnumDef => {
                        let mut e = RbqEnum::lower(node, source);
                        e.annotations.extend(pending_annotations.drain(..));
                        items.push(RbqItem::Enum(e))
                    }
                    RbqTokenType::UnionDef => {
                        let mut u = RbqUnion::lower(node, source);
                        u.annotations.extend(pending_annotations.drain(..));
                        items.push(RbqItem::Union(u))
                    }
                    RbqTokenType::TraitDef => {
                        let mut t = RbqTrait::lower(node, source);
                        t.annotations.extend(pending_annotations.drain(..));
                        items.push(RbqItem::Trait(t))
                    }
                    RbqTokenType::TypeDef => {
                        let mut t = RbqTypeAlias::lower(node, source);
                        t.annotations.extend(pending_annotations.drain(..));
                        items.push(RbqItem::TypeAlias(t))
                    }
                    RbqTokenType::MicroDef => {
                        let mut m = RbqMicro::lower(node, source);
                        m.annotations.extend(pending_annotations.drain(..));
                        items.push(RbqItem::Micro(m))
                    }
                    RbqTokenType::QueryPipeline | RbqTokenType::Expression => items.push(RbqItem::Query(RbqExpr::lower(node, source))),
                    _ => {}
                },
                RedTree::Token(_) => {}
            }
        }
        Self { items }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RbqItem {
    Struct(RbqStruct),
    Union(RbqUnion),
    Enum(RbqEnum),
    Trait(RbqTrait),
    Namespace(RbqNamespace),
    TypeAlias(RbqTypeAlias),
    Micro(RbqMicro),
    Query(RbqExpr),
    Import(RbqImport),
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RbqImport {
    pub path: String,
}

impl RbqImport {
    pub fn lower(red: RedNode<RbqLanguage>, source: &str) -> Self {
        let mut path = String::new();
        for child in red.children() {
            if child.kind::<RbqTokenType>() == RbqTokenType::Ident || child.kind::<RbqTokenType>() == RbqTokenType::Utf8Kw || child.kind::<RbqTokenType>() == RbqTokenType::Dot {
                path.push_str(source[child.span()].trim());
            }
        }
        Self { path }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RbqStruct {
    pub annotations: Vec<RbqAnnotation>,
    pub name: String,
    pub fields: Vec<RbqField>,
}

impl RbqStruct {
    pub fn lower(red: RedNode<RbqLanguage>, source: &str) -> Self {
        let mut annotations = Vec::new();
        let mut name = String::new();
        let mut fields = Vec::new();

        for child in red.children() {
            match child.kind::<RbqTokenType>() {
                RbqTokenType::Annotation => {
                    if let Some(node) = child.as_node() {
                        annotations.push(RbqAnnotation::lower(node, source))
                    }
                }
                RbqTokenType::Ident if name.is_empty() => name = source[child.span()].trim().to_string(),
                RbqTokenType::FieldDef => {
                    if let Some(node) = child.as_node() {
                        fields.push(RbqField::lower(node, source))
                    }
                }
                _ => {}
            }
        }

        Self { annotations, name, fields }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RbqNamespace {
    pub name: String,
    pub items: Vec<RbqItem>,
}

impl RbqNamespace {
    pub fn lower(red: RedNode<RbqLanguage>, source: &str) -> Self {
        let mut name = String::new();
        let mut items = Vec::new();
        let mut pending_annotations = Vec::new();

        for child in red.children() {
            match child.kind::<RbqTokenType>() {
                RbqTokenType::Ident if name.is_empty() => name = source[child.span()].trim().to_string(),
                RbqTokenType::Annotation => {
                    if let Some(node) = child.as_node() {
                        pending_annotations.push(RbqAnnotation::lower(node, source))
                    }
                }
                RbqTokenType::StructDef | RbqTokenType::ClassDef => {
                    if let Some(node) = child.as_node() {
                        let mut s = RbqStruct::lower(node, source);
                        s.annotations.extend(pending_annotations.drain(..));
                        items.push(RbqItem::Struct(s))
                    }
                }
                RbqTokenType::UnionDef => {
                    if let Some(node) = child.as_node() {
                        let mut u = RbqUnion::lower(node, source);
                        u.annotations.extend(pending_annotations.drain(..));
                        items.push(RbqItem::Union(u))
                    }
                }
                RbqTokenType::EnumDef => {
                    if let Some(node) = child.as_node() {
                        let mut e = RbqEnum::lower(node, source);
                        e.annotations.extend(pending_annotations.drain(..));
                        items.push(RbqItem::Enum(e))
                    }
                }
                RbqTokenType::TraitDef => {
                    if let Some(node) = child.as_node() {
                        let mut t = RbqTrait::lower(node, source);
                        t.annotations.extend(pending_annotations.drain(..));
                        items.push(RbqItem::Trait(t))
                    }
                }
                RbqTokenType::TypeDef => {
                    if let Some(node) = child.as_node() {
                        let mut t = RbqTypeAlias::lower(node, source);
                        t.annotations.extend(pending_annotations.drain(..));
                        items.push(RbqItem::TypeAlias(t))
                    }
                }
                RbqTokenType::MicroDef => {
                    if let Some(node) = child.as_node() {
                        items.push(RbqItem::Micro(RbqMicro::lower(node, source)))
                    }
                }
                RbqTokenType::NamespaceDef => {
                    if let Some(node) = child.as_node() {
                        items.push(RbqItem::Namespace(RbqNamespace::lower(node, source)))
                    }
                }
                RbqTokenType::ImportDef => {
                    if let Some(node) = child.as_node() {
                        items.push(RbqItem::Import(RbqImport::lower(node, source)))
                    }
                }
                RbqTokenType::QueryPipeline | RbqTokenType::Expression => {
                    if let Some(node) = child.as_node() {
                        items.push(RbqItem::Query(RbqExpr::lower(node, source)))
                    }
                }
                _ => {}
            }
        }

        Self { name, items }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RbqUnion {
    pub annotations: Vec<RbqAnnotation>,
    pub name: String,
    pub members: Vec<RbqUnionMember>,
}

impl RbqUnion {
    pub fn lower(red: RedNode<RbqLanguage>, source: &str) -> Self {
        let mut annotations = Vec::new();
        let mut name = String::new();
        let mut members = Vec::new();

        for child in red.children() {
            match child {
                RedTree::Node(node) => match node.kind::<RbqTokenType>() {
                    RbqTokenType::Annotation => annotations.push(RbqAnnotation::lower(node, source)),
                    RbqTokenType::UnionMember | RbqTokenType::EnumMember => members.push(RbqUnionMember::lower(node, source)),
                    RbqTokenType::MicroDef => {
                        // Handle micro defs in unions if they are methods
                    }
                    _ => {}
                },
                RedTree::Token(leaf) => {
                        if leaf.kind() == RbqTokenType::Ident && name.is_empty() {
                            name = source[leaf.span()].trim().to_string();
                        }
                    }
            }
        }

        Self { annotations, name, members }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RbqEnum {
    pub annotations: Vec<RbqAnnotation>,
    pub name: String,
    pub using: Vec<String>,
    pub variants: Vec<RbqEnumMember>,
}

impl RbqEnum {
    pub fn lower(red: RedNode<RbqLanguage>, source: &str) -> Self {
        let mut annotations = Vec::new();
        let mut name = String::new();
        let mut using = Vec::new();
        let mut variants = Vec::new();

        for child in red.children() {
            match child {
                RedTree::Node(node) => match node.kind::<RbqTokenType>() {
                    RbqTokenType::Annotation => annotations.push(RbqAnnotation::lower(node, source)),
                    RbqTokenType::UsingDef => {
                        for c in node.children() {
                            if c.kind::<RbqTokenType>() == RbqTokenType::Ident || c.kind::<RbqTokenType>() == RbqTokenType::Utf8Kw {
                                using.push(source[c.span()].to_string());
                            }
                        }
                    }
                    RbqTokenType::EnumMember => variants.push(RbqEnumMember::lower(node, source)),
                    _ => {}
                },
                RedTree::Token(leaf) => {
                        if leaf.kind() == RbqTokenType::Ident && name.is_empty() {
                            name = source[leaf.span()].trim().to_string();
                        }
                    }
            }
        }

        Self { annotations, name, using, variants }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RbqEnumMember {
    pub annotations: Vec<RbqAnnotation>,
    pub name: String,
    pub value: Option<String>,
}

impl RbqEnumMember {
    pub fn lower(red: RedNode<RbqLanguage>, source: &str) -> Self {
        let mut annotations = Vec::new();
        let mut name = String::new();
        let mut value = None;

        for child in red.children() {
            match child.kind::<RbqTokenType>() {
                RbqTokenType::Annotation => {
                    if let Some(node) = child.as_node() {
                        annotations.push(RbqAnnotation::lower(node, source))
                    }
                }
                RbqTokenType::Ident if name.is_empty() => name = source[child.span()].trim().to_string(),
                RbqTokenType::Literal => value = Some(source[child.span()].trim().to_string()),
                _ => {}
            }
        }

        Self { annotations, name, value }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RbqUnionMember {
    pub annotations: Vec<RbqAnnotation>,
    pub name: String,
    pub payload: Option<RbqUnionPayload>,
    pub value: Option<String>,
}

impl RbqUnionMember {
    pub fn lower(red: RedNode<RbqLanguage>, source: &str) -> Self {
        let mut annotations = Vec::new();
        let mut name = String::new();
        let mut payload = None;
        let mut value = None;

        for child in red.children() {
            match child.kind() {
                RbqTokenType::Annotation => {
                    if let Some(node) = child.as_node() {
                        annotations.push(RbqAnnotation::lower(node, source))
                    }
                }
                RbqTokenType::Ident if name.is_empty() => name = source[child.span()].trim().to_string(),
                RbqTokenType::FieldDef => {
                    if let Some(node) = child.as_node() {
                        if payload.is_none() {
                            payload = Some(RbqUnionPayload::Struct(Vec::new()))
                        }
                        if let Some(RbqUnionPayload::Struct(fields)) = &mut payload {
                            fields.push(RbqField::lower(node, source))
                        }
                    }
                }
                RbqTokenType::TypeRef => {
                    if let Some(node) = child.as_node() {
                        if payload.is_none() {
                            payload = Some(RbqUnionPayload::Tuple(Vec::new()))
                        }
                        if let Some(RbqUnionPayload::Tuple(types)) = &mut payload {
                            types.push(RbqType::lower(node, source))
                        }
                    }
                }
                RbqTokenType::Literal => value = Some(source[child.span()].trim().to_string()),
                _ => {}
            }
        }

        Self { annotations, name, payload, value }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RbqUnionPayload {
    Tuple(Vec<RbqType>),
    Struct(Vec<RbqField>),
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RbqTrait {
    pub annotations: Vec<RbqAnnotation>,
    pub name: String,
    pub items: Vec<RbqTraitItem>,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RbqTraitItem {
    Field(RbqField),
    Method(RbqMicro),
}

impl RbqTrait {
    pub fn lower(red: RedNode<RbqLanguage>, source: &str) -> Self {
        let mut annotations = Vec::new();
        let mut name = String::new();
        let mut items = Vec::new();

        for child in red.children() {
            match child.kind() {
                RbqTokenType::Annotation => {
                    if let Some(node) = child.as_node() {
                        annotations.push(RbqAnnotation::lower(node, source))
                    }
                }
                RbqTokenType::Ident if name.is_empty() => name = source[child.span()].trim().to_string(),
                RbqTokenType::FieldDef => {
                    if let Some(node) = child.as_node() {
                        items.push(RbqTraitItem::Field(RbqField::lower(node, source)))
                    }
                }
                RbqTokenType::MicroDef => {
                    if let Some(node) = child.as_node() {
                        items.push(RbqTraitItem::Method(RbqMicro::lower(node, source)))
                    }
                }
                _ => {}
            }
        }

        Self { annotations, name, items }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RbqField {
    pub annotations: Vec<RbqAnnotation>,
    pub name: String,
    pub type_ref: RbqType,
    pub default_value: Option<RbqExpr>,
}

impl RbqField {
    pub fn lower(red: RedNode<RbqLanguage>, source: &str) -> Self {
        let mut annotations = Vec::new();
        let mut name = String::new();
        let mut type_ref = None;
        let mut default_value = None;

        for child in red.children() {
            match child.kind::<RbqTokenType>() {
                RbqTokenType::Annotation => {
                    if let Some(node) = child.as_node() {
                        annotations.push(RbqAnnotation::lower(node, source))
                    }
                }
                RbqTokenType::Ident if name.is_empty() => name = source[child.span()].trim().to_string(),
                RbqTokenType::TypeDef | RbqTokenType::TypeRef => {
                    if let Some(node) = child.as_node() {
                        type_ref = Some(RbqType::lower(node, source))
                    }
                }
                _ => {
                    if let Some(node) = child.as_node() {
                        if node.kind::<RbqTokenType>() == RbqTokenType::Expression {
                            default_value = Some(RbqExpr::lower(node, source))
                        }
                    }
                }
            }
        }

        Self {
            annotations,
            name,
            type_ref: type_ref.unwrap_or(RbqType::Named {
                path: "any".to_string(),
                generic_args: Vec::new(),
                is_physical_ptr: false,
                is_optional: false,
            }),
            default_value,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RbqTypeAlias {
    pub annotations: Vec<RbqAnnotation>,
    pub name: String,
    pub type_ref: RbqType,
}

impl RbqTypeAlias {
    pub fn lower(red: RedNode<RbqLanguage>, source: &str) -> Self {
        let mut annotations = Vec::new();
        let mut name = String::new();
        let mut type_ref = RbqType::Named { 
            path: "unknown".to_string(), 
            generic_args: Vec::new(), 
            is_physical_ptr: false, 
            is_optional: false 
        };

        for child in red.children() {
            match child.kind::<RbqTokenType>() {
                RbqTokenType::Annotation => {
                    if let Some(node) = child.as_node() {
                        annotations.push(RbqAnnotation::lower(node, source))
                    }
                }
                RbqTokenType::Ident if name.is_empty() => name = source[child.span()].trim().to_string(),
                RbqTokenType::TypeRef | RbqTokenType::TypeDef => {
                    if let Some(node) = child.as_node() {
                        type_ref = RbqType::lower(node, source)
                    }
                }
                _ => {}
            }
        }

        Self { annotations, name, type_ref }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RbqType {
    Named {
        path: String,
        generic_args: Vec<RbqType>,
        is_physical_ptr: bool,
        is_optional: bool,
    },
    InlineStruct(Vec<RbqField>),
    PhysicalRef(Box<RbqType>),
    Optional(Box<RbqType>),
    Literal(String),
}

impl RbqType {
    pub fn lower(red: RedNode<RbqLanguage>, source: &str) -> Self {
        let mut is_optional = false;
        let mut is_ref = false;
        let mut path = String::new();
        let mut generic_args = Vec::new();
        let mut inline_fields = Vec::new();
        let mut literal = None;

        for child in red.children() {
            match child.kind::<RbqTokenType>() {
                RbqTokenType::Question => is_optional = true,
                RbqTokenType::Ampersand => is_ref = true,
                RbqTokenType::Ident | RbqTokenType::Utf8Kw => {
                    if path.is_empty() {
                        path = source[child.span()].trim().to_string();
                    }
                    else {
                        path.push('.');
                        path.push_str(source[child.span()].trim());
                    }
                }
                RbqTokenType::Dot => {}
                RbqTokenType::GenericArgs => {
                    if let Some(node) = child.as_node() {
                        for arg_child in node.children() {
                            if let Some(arg_node) = arg_child.as_node() {
                                generic_args.push(RbqType::lower(arg_node, source));
                            }
                            else if arg_child.kind::<RbqTokenType>() == RbqTokenType::Literal {
                                generic_args.push(RbqType::Literal(source[arg_child.span()].trim().to_string()));
                            }
                        }
                    }
                }
                RbqTokenType::FieldDef => {
                    if let Some(node) = child.as_node() {
                        inline_fields.push(RbqField::lower(node, source));
                    }
                }
                RbqTokenType::Literal => {
                    literal = Some(source[child.span()].trim().to_string());
                }
                _ => {}
            }
        }

        if !inline_fields.is_empty() {
            let mut res = RbqType::InlineStruct(inline_fields);
            if is_ref { res = RbqType::PhysicalRef(Box::new(res)); }
            if is_optional { res = RbqType::Optional(Box::new(res)); }
            res
        }
        else if let Some(lit) = literal {
            RbqType::Literal(lit)
        }
        else {
            RbqType::Named { path, generic_args, is_physical_ptr: is_ref, is_optional }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RbqAnnotation {
    pub name: String,
    pub args: Vec<String>,
}

impl RbqAnnotation {
    pub fn lower(red: RedNode<RbqLanguage>, source: &str) -> Self {
        let mut name = String::new();
        let mut args = Vec::new();

        for child in red.children() {
            match child.kind::<RbqTokenType>() {
                RbqTokenType::Ident if name.is_empty() => name = source[child.span()].trim().to_string(),
                RbqTokenType::AnnotationArgs => {
                    if let Some(node) = child.as_node() {
                        for arg_child in node.children() {
                            match arg_child.kind::<RbqTokenType>() {
                                RbqTokenType::Literal | RbqTokenType::MagicVar | RbqTokenType::BinaryExpr | RbqTokenType::MemberExpr | RbqTokenType::CallExpr | RbqTokenType::Ident => args.push(source[arg_child.span()].trim().to_string()),
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        Self { name, args }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RbqMicro {
    pub annotations: Vec<RbqAnnotation>,
    pub name: String,
    pub args: Vec<(String, RbqType)>,
    pub return_type: Option<RbqType>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl RbqMicro {
    pub fn lower(red: RedNode<RbqLanguage>, source: &str) -> Self {
        let span = red.span();
        let mut annotations = Vec::new();
        let mut name = String::new();
        let mut args = Vec::new();
        let mut return_type = None;

        for child in red.children() {
            match child.kind() {
                RbqTokenType::Annotation => {
                    if let Some(node) = child.as_node() {
                        annotations.push(RbqAnnotation::lower(node, source))
                    }
                }
                RbqTokenType::Ident if name.is_empty() => name = source[child.span()].trim().to_string(),
                RbqTokenType::FieldDef => {
                    if let Some(node) = child.as_node() {
                        let field = RbqField::lower(node, source);
                        args.push((field.name, field.type_ref));
                    }
                }
                RbqTokenType::TypeRef => {
                    if let Some(node) = child.as_node() {
                        return_type = Some(RbqType::lower(node, source));
                    }
                }
                _ => {}
            }
        }

        Self { annotations, name, args, return_type, span }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RbqExpr {
    pub kind: RbqExprKind,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RbqExprKind {
    Literal(RbqLiteral),
    Identifier(String),
    Binary { left: Box<RbqExpr>, op: String, right: Box<RbqExpr> },
    Unary { op: String, expr: Box<RbqExpr> },
    Call { callee: Box<RbqExpr>, args: Vec<RbqExpr> },
    Member { object: Box<RbqExpr>, property: String },
    Pipeline { base: Box<RbqExpr>, steps: Vec<RbqPipelineStep> },
    Closure { args: Vec<String>, body: Box<RbqExpr> },
    Block(Vec<RbqExpr>),
    MagicVar(String), // $, $key, $group
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RbqLiteral {
    String(String),
    Number(String),
    Boolean(bool),
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RbqPipelineStep {
    pub name: String,
    pub args: Vec<RbqExpr>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl RbqExpr {
    pub fn lower(red: RedNode<RbqLanguage>, source: &str) -> Self {
        let span = red.span();
        let kind = match red.kind() {
            RbqTokenType::Literal => {
                let text = source[span.clone()].trim().to_string();
                if let Some(leaf) = red.children().find_map(|c| c.as_token()) {
                    match leaf.kind() {
                        RbqTokenType::StringLiteral => RbqExprKind::Literal(RbqLiteral::String(text)),
                        RbqTokenType::NumberLiteral => RbqExprKind::Literal(RbqLiteral::Number(text)),
                        RbqTokenType::TrueKw => RbqExprKind::Literal(RbqLiteral::Boolean(true)),
                        RbqTokenType::FalseKw => RbqExprKind::Literal(RbqLiteral::Boolean(false)),
                        _ => RbqExprKind::Literal(RbqLiteral::String(text)),
                    }
                }
                else {
                    RbqExprKind::Literal(RbqLiteral::String(text))
                }
            }
            RbqTokenType::Ident => {
                let text = source[span.clone()].trim().to_string();
                RbqExprKind::Identifier(text)
            }
            RbqTokenType::MagicVar => {
                let text = source[span.clone()].trim().to_string();
                RbqExprKind::MagicVar(text)
            }
            RbqTokenType::BinaryExpr => {
                let mut left = None;
                let mut op = String::new();
                let mut right = None;
                for child in red.children() {
                    match child {
                        RedTree::Node(node) => {
                            if left.is_none() {
                                left = Some(Box::new(RbqExpr::lower(node, source)))
                            }
                            else {
                                right = Some(Box::new(RbqExpr::lower(node, source)))
                            }
                        }
                        RedTree::Token(leaf) => {
                            let k = leaf.kind();
                            if TokenType::role(&k) == oak_core::UniversalTokenRole::Operator {
                                op = source[leaf.span()].trim().to_string()
                            }
                        }
                    }
                }
                if let (Some(left), Some(right)) = (left, right) { RbqExprKind::Binary { left, op, right } } else { RbqExprKind::Identifier(source[span.clone()].to_string()) }
            }
            RbqTokenType::UnaryExpr => {
                let mut op = String::new();
                let mut expr = None;
                for child in red.children() {
                    match child {
                        RedTree::Node(node) => expr = Some(Box::new(RbqExpr::lower(node, source))),
                        RedTree::Token(leaf) => {
                            let k = leaf.kind();
                            if TokenType::role(&k) == oak_core::UniversalTokenRole::Operator {
                                op = source[leaf.span()].trim().to_string()
                            }
                        }
                    }
                }
                if let Some(expr) = expr { RbqExprKind::Unary { op, expr } } else { RbqExprKind::Identifier(source[span.clone()].to_string()) }
            }
            RbqTokenType::CallExpr => {
                let mut callee = None;
                let mut args = Vec::new();
                for child in red.children() {
                    match child {
                        RedTree::Node(node) => {
                            if callee.is_none() {
                                callee = Some(Box::new(RbqExpr::lower(node, source)))
                            }
                            else {
                                args.push(RbqExpr::lower(node, source))
                            }
                        }
                        _ => {}
                    }
                }
                if let Some(callee) = callee { RbqExprKind::Call { callee, args } } else { RbqExprKind::Identifier(source[span.clone()].to_string()) }
            }
            RbqTokenType::MemberExpr => {
                let mut object = None;
                let mut property = String::new();
                for child in red.children() {
                    match child {
                        RedTree::Node(node) => object = Some(Box::new(RbqExpr::lower(node, source))),
                        RedTree::Token(leaf) if leaf.kind() == RbqTokenType::Ident => property = source[leaf.span()].to_string(),
                        _ => {}
                    }
                }
                if let Some(object) = object { RbqExprKind::Member { object, property } } else { RbqExprKind::Identifier(source[span.clone()].to_string()) }
            }
            RbqTokenType::QueryPipeline => {
                let mut base = None;
                let mut steps = Vec::new();
                for child in red.children() {
                    match child {
                        RedTree::Node(node) => {
                            if node.kind::<RbqTokenType>() == RbqTokenType::PipelineStep {
                                steps.push(RbqPipelineStep::lower(node, source))
                            }
                            else if base.is_none() {
                                base = Some(Box::new(RbqExpr::lower(node, source)))
                            }
                        }
                        _ => {}
                    }
                }
                if let Some(base) = base { RbqExprKind::Pipeline { base, steps } } else { RbqExprKind::Identifier(source[span.clone()].to_string()) }
            }
            RbqTokenType::Closure => {
                let mut args = Vec::new();
                let mut body = None;
                for child in red.children() {
            match child {
                RedTree::Node(node) => match node.kind::<RbqTokenType>() {
                            RbqTokenType::ClosureArgs => {
                                for arg in node.children() {
                                    if let RedTree::Token(leaf) = arg {
                                        if leaf.kind() == RbqTokenType::Ident {
                                            args.push(source[leaf.span()].to_string())
                                        }
                                    }
                                }
                            }
                            _ => body = Some(Box::new(RbqExpr::lower(node, source))),
                        },
                        _ => {}
                    }
                }
                if let Some(body) = body { RbqExprKind::Closure { args, body } } else { RbqExprKind::Identifier(source[span.clone()].to_string()) }
            }
            RbqTokenType::Block => {
                let mut expressions = Vec::new();
                for child in red.children() {
                    if let RedTree::Node(node) = child {
                        expressions.push(RbqExpr::lower(node, source));
                    }
                }
                RbqExprKind::Block(expressions)
            }
            RbqTokenType::Expression => {
                let first_node = red.children().find_map(|c| c.as_node());
                if let Some(node) = first_node { return Self::lower(node, source) } else { RbqExprKind::Identifier(source[span.clone()].to_string()) }
            }
            _ => RbqExprKind::Identifier(source[span.clone()].to_string()),
        };

        Self { kind, span }
    }
}

impl RbqPipelineStep {
    pub fn lower(red: RedNode<RbqLanguage>, source: &str) -> Self {
        let span = red.span();
        let mut name = String::new();
        let mut args = Vec::new();
        for child in red.children() {
            match child {
                RedTree::Node(node) => args.push(RbqExpr::lower(node, source)),
                RedTree::Token(leaf) if leaf.kind::<RbqTokenType>() == RbqTokenType::Ident && name.is_empty() => name = source[leaf.span()].to_string(),
                _ => {}
            }
        }
        Self { name, args, span }
    }
}
