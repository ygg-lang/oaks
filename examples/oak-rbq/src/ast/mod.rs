#![doc = include_str!("readme.md")]
use crate::{language::RbqLanguage, lexer::token_type::RbqTokenType, parser::element_type::RbqElementType};
use core::range::Range;
use oak_core::{
    ElementType, TokenType,
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
                RedTree::Node(node) => match node.kind::<RbqElementType>() {
                    RbqElementType::Annotation => pending_annotations.push(RbqAnnotation::lower(node, source)),
                    RbqElementType::NamespaceDef => {
                        let mut ns = RbqNamespace::lower(node, source);
                        ns.annotations.extend(pending_annotations.drain(..));
                        items.push(RbqItem::Namespace(ns))
                    }
                    RbqElementType::ImportDef => {
                        pending_annotations.clear();
                        items.push(RbqItem::Import(RbqImport::lower(node, source)))
                    }
                    RbqElementType::StructDef | RbqElementType::ClassDef => {
                        let mut s = RbqStruct::lower(node, source);
                        s.annotations.extend(pending_annotations.drain(..));
                        items.push(RbqItem::Struct(s))
                    }
                    RbqElementType::EnumDef => {
                        let mut e = RbqEnum::lower(node, source);
                        e.annotations.extend(pending_annotations.drain(..));
                        items.push(RbqItem::Enum(e))
                    }
                    RbqElementType::UnionDef => {
                        let mut u = RbqUnion::lower(node, source);
                        u.annotations.extend(pending_annotations.drain(..));
                        items.push(RbqItem::Union(u))
                    }
                    RbqElementType::TraitDef => {
                        let mut t = RbqTrait::lower(node, source);
                        t.annotations.extend(pending_annotations.drain(..));
                        items.push(RbqItem::Trait(t))
                    }
                    RbqElementType::TypeDef => {
                        let mut t = RbqTypeAlias::lower(node, source);
                        t.annotations.extend(pending_annotations.drain(..));
                        items.push(RbqItem::TypeAlias(t))
                    }
                    RbqElementType::MicroDef => {
                        let mut m = RbqMicro::lower(node, source);
                        m.annotations.extend(pending_annotations.drain(..));
                        items.push(RbqItem::Micro(m))
                    }
                    RbqElementType::QueryPipeline | RbqElementType::Closure | RbqElementType::BinaryExpr | RbqElementType::Expression | RbqElementType::CallExpr | RbqElementType::MemberExpr | RbqElementType::Literal | RbqElementType::MagicVar => {
                        let expr = RbqExpr::lower(node, source);
                        // If it's query = ..., just take the right side
                        match &expr.kind {
                            RbqExprKind::Binary { left, op, right } if op == "=" => {
                                if let RbqExprKind::Identifier(id) = &left.kind {
                                    if id == "query" {
                                        items.push(RbqItem::Query(*right.clone()));
                                        continue;
                                    }
                                }
                            }
                            _ => {}
                        }
                        items.push(RbqItem::Query(expr));
                    }
                    RbqElementType::Eof | RbqElementType::Error => {}
                    _ => {}
                },
                RedTree::Leaf(_) => {}
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
            if child.kind::<RbqElementType>() == RbqElementType::Ident || child.kind::<RbqElementType>() == RbqElementType::Utf8Kw || child.kind::<RbqElementType>() == RbqElementType::Dot {
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

        Self { annotations, name, fields }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RbqNamespace {
    pub annotations: Vec<RbqAnnotation>,
    pub name: String,
    pub items: Vec<RbqItem>,
}

impl RbqNamespace {
    pub fn lower(red: RedNode<RbqLanguage>, source: &str) -> Self {
        let mut annotations = Vec::new();
        let mut name = String::new();
        let mut items = Vec::new();
        let mut pending_annotations = Vec::new();

        for child in red.children() {
            match child.kind::<RbqElementType>() {
                RbqElementType::Ident => {
                    if name.is_empty() {
                        name = source[child.span()].trim().to_string();
                    }
                    else {
                        name.push_str(source[child.span()].trim());
                    }
                }
                RbqElementType::Utf8Kw => {
                    if name.is_empty() {
                        name = source[child.span()].trim().to_string();
                    }
                    else {
                        name.push_str(source[child.span()].trim());
                    }
                }
                RbqElementType::Dot => {
                    if !name.is_empty() {
                        name.push('.');
                    }
                }
                RbqElementType::Annotation => {
                    if let Some(node) = child.as_node() {
                        let ann = RbqAnnotation::lower(node, source);
                        if name.is_empty() {
                            annotations.push(ann);
                        }
                        else {
                            pending_annotations.push(ann);
                        }
                    }
                }
                RbqElementType::StructDef | RbqElementType::ClassDef => {
                    if let Some(node) = child.as_node() {
                        let mut s = RbqStruct::lower(node, source);
                        s.annotations.extend(pending_annotations.drain(..));
                        items.push(RbqItem::Struct(s))
                    }
                }
                RbqElementType::UnionDef => {
                    if let Some(node) = child.as_node() {
                        let mut u = RbqUnion::lower(node, source);
                        u.annotations.extend(pending_annotations.drain(..));
                        items.push(RbqItem::Union(u))
                    }
                }
                RbqElementType::EnumDef => {
                    if let Some(node) = child.as_node() {
                        let mut e = RbqEnum::lower(node, source);
                        e.annotations.extend(pending_annotations.drain(..));
                        items.push(RbqItem::Enum(e))
                    }
                }
                RbqElementType::TraitDef => {
                    if let Some(node) = child.as_node() {
                        let mut t = RbqTrait::lower(node, source);
                        t.annotations.extend(pending_annotations.drain(..));
                        items.push(RbqItem::Trait(t))
                    }
                }
                RbqElementType::TypeDef => {
                    if let Some(node) = child.as_node() {
                        let mut t = RbqTypeAlias::lower(node, source);
                        t.annotations.extend(pending_annotations.drain(..));
                        items.push(RbqItem::TypeAlias(t))
                    }
                }
                RbqElementType::MicroDef => {
                    if let Some(node) = child.as_node() {
                        let mut m = RbqMicro::lower(node, source);
                        m.annotations.extend(pending_annotations.drain(..));
                        items.push(RbqItem::Micro(m))
                    }
                }
                RbqElementType::NamespaceDef => {
                    if let Some(node) = child.as_node() {
                        let mut ns = RbqNamespace::lower(node, source);
                        ns.annotations.extend(pending_annotations.drain(..));
                        items.push(RbqItem::Namespace(ns))
                    }
                }
                RbqElementType::ImportDef => {
                    if let Some(node) = child.as_node() {
                        pending_annotations.clear();
                        items.push(RbqItem::Import(RbqImport::lower(node, source)))
                    }
                }
                RbqElementType::QueryPipeline
                | RbqElementType::Closure
                | RbqElementType::BinaryExpr
                | RbqElementType::Expression
                | RbqElementType::CallExpr
                | RbqElementType::MemberExpr
                | RbqElementType::Literal
                | RbqElementType::MagicVar
                | RbqElementType::Ident => {
                    if let Some(node) = child.as_node() {
                        items.push(RbqItem::Query(RbqExpr::lower(node, source)))
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
                RedTree::Node(node) => match node.kind::<RbqElementType>() {
                    RbqElementType::Annotation => annotations.push(RbqAnnotation::lower(node, source)),
                    RbqElementType::UnionMember | RbqElementType::EnumMember => members.push(RbqUnionMember::lower(node, source)),
                    RbqElementType::MicroDef => {
                        // Handle micro defs in unions if they are methods
                    }
                    _ => {}
                },
                RedTree::Leaf(leaf) if leaf.kind() == RbqTokenType::Ident && name.is_empty() => name = source[leaf.span()].trim().to_string(),
                RedTree::Leaf(_) => {}
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
                RedTree::Node(node) => match node.kind::<RbqElementType>() {
                    RbqElementType::Annotation => annotations.push(RbqAnnotation::lower(node, source)),
                    RbqElementType::UsingDef => {
                        for c in node.children() {
                            if c.kind::<RbqElementType>() == RbqElementType::Ident || c.kind::<RbqElementType>() == RbqElementType::Utf8Kw {
                                using.push(source[c.span()].to_string());
                            }
                        }
                    }
                    RbqElementType::EnumMember => variants.push(RbqEnumMember::lower(node, source)),
                    _ => {}
                },
                RedTree::Leaf(leaf) if leaf.kind() == RbqTokenType::Ident && name.is_empty() => name = source[leaf.span()].trim().to_string(),
                RedTree::Leaf(_) => {}
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
            match child.kind::<RbqElementType>() {
                RbqElementType::Annotation => {
                    if let Some(node) = child.as_node() {
                        annotations.push(RbqAnnotation::lower(node, source))
                    }
                }
                RbqElementType::Ident if name.is_empty() => name = source[child.span()].trim().to_string(),
                RbqElementType::Literal => {
                    let text = source[child.span()].trim();
                    if text.starts_with('"') || text.starts_with('\'') {
                        // Strip quotes
                        value = Some(text[1..text.len()-1].to_string());
                    } else if text == "true" || text == "false" {
                        value = Some(text.to_string());
                    } else if text.chars().next().map_or(false, |c| c.is_ascii_digit()) {
                        value = Some(text.to_string());
                    }
                }
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
            match child.kind::<RbqElementType>() {
                RbqElementType::Annotation => {
                    if let Some(node) = child.as_node() {
                        annotations.push(RbqAnnotation::lower(node, source))
                    }
                }
                RbqElementType::Ident if name.is_empty() => name = source[child.span()].trim().to_string(),
                RbqElementType::FieldDef => {
                    if let Some(node) = child.as_node() {
                        if payload.is_none() {
                            payload = Some(RbqUnionPayload::Struct(Vec::new()))
                        }
                        if let Some(RbqUnionPayload::Struct(fields)) = &mut payload {
                            fields.push(RbqField::lower(node, source))
                        }
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
            match child.kind::<RbqElementType>() {
                RbqElementType::Annotation => {
                    if let Some(node) = child.as_node() {
                        annotations.push(RbqAnnotation::lower(node, source))
                    }
                }
                RbqElementType::MicroDef => {
                    if let Some(node) = child.as_node() {
                        items.push(RbqTraitItem::Method(RbqMicro::lower(node, source)))
                    }
                }
                RbqElementType::Ident if name.is_empty() => name = source[child.span()].trim().to_string(),
                RbqElementType::FieldDef => {
                    if let Some(node) = child.as_node() {
                        let field = RbqField::lower(node, source);
                        // If it's a field with a Micro value, it's a method
                        if let Some(expr) = &field.default_value {
                            if let RbqExprKind::Identifier(id) = &expr.kind {
                                if id == "micro" {
                                    // This happens when we have `print: micro();` in a trait
                                    // We should convert it to a method
                                    let mut micro = RbqMicro { annotations: field.annotations, name: field.name, args: Vec::new(), return_type: None, body: None, span: node.span() };

                                    // Try to find the actual micro definition if it was parsed as a sibling or child
                                    // In current parser, `micro(...) -> ...` might be a CallExpr if it's in a default value
                                    // But for now, let's just use the field info
                                    items.push(RbqTraitItem::Method(micro));
                                    continue;
                                }
                            }
                            else if let RbqExprKind::Call { callee, args } = &expr.kind {
                                if let RbqExprKind::Identifier(id) = &callee.kind {
                                    if id == "micro" {
                                        // Handle `print: micro(token: string) -> bool;`
                                        // Extract return type from the original source or node if possible
                                        // For now, a simplified conversion:
                                        let mut micro = RbqMicro {
                                            annotations: field.annotations,
                                            name: field.name,
                                            args: Vec::new(),  // TODO: extract from CallExpr args
                                            return_type: None, // TODO: extract from somewhere
                                            body: None,
                                            span: node.span(),
                                        };

                                        // Try to map CallExpr args to RbqField args
                                        for arg_expr in args {
                                            if let RbqExprKind::Binary { left, op, right } = &arg_expr.kind {
                                                if op == ":" {
                                                    if let RbqExprKind::Identifier(arg_name) = &left.kind {
                                                        // simplified type extraction
                                                        let type_path = source[right.span].trim().to_string();
                                                        micro.args.push(RbqField {
                                                            annotations: Vec::new(),
                                                            name: arg_name.clone(),
                                                            type_ref: RbqType::Named { path: type_path, generic_args: Vec::new(), is_physical_ptr: false, is_optional: false },
                                                            default_value: None,
                                                        });
                                                    }
                                                }
                                            }
                                        }

                                        items.push(RbqTraitItem::Method(micro));
                                        continue;
                                    }
                                }
                            }
                        }
                        items.push(RbqTraitItem::Field(field))
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

        Self { annotations, name, type_ref: type_ref.unwrap_or(RbqType::Named { path: "any".to_string(), generic_args: Vec::new(), is_physical_ptr: false, is_optional: false }), default_value }
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
        let mut type_ref = RbqType::Named { path: "unknown".to_string(), generic_args: Vec::new(), is_physical_ptr: false, is_optional: false };

        for child in red.children() {
            match child.kind::<RbqElementType>() {
                RbqElementType::Annotation => {
                    if let Some(node) = child.as_node() {
                        annotations.push(RbqAnnotation::lower(node, source))
                    }
                }
                RbqElementType::Ident if name.is_empty() => name = source[child.span()].trim().to_string(),
                RbqElementType::TypeRef | RbqElementType::TypeDef => {
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
    Named { path: String, generic_args: Vec<RbqType>, is_physical_ptr: bool, is_optional: bool },
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
            match child.kind::<RbqElementType>() {
                RbqElementType::Question => is_optional = true,
                RbqElementType::Ampersand => is_ref = true,
                RbqElementType::Ident | RbqElementType::Utf8Kw => {
                    if path.is_empty() {
                        path = source[child.span()].trim().to_string();
                    }
                    else {
                        path.push('.');
                        path.push_str(source[child.span()].trim());
                    }
                }
                RbqElementType::Dot => {}
                RbqElementType::GenericArgs => {
                    if let Some(node) = child.as_node() {
                        for arg_child in node.children() {
                            if let Some(arg_node) = arg_child.as_node() {
                                generic_args.push(RbqType::lower(arg_node, source));
                            }
                            else if arg_child.kind::<RbqElementType>() == RbqElementType::NumberLiteral || arg_child.kind::<RbqElementType>() == RbqElementType::StringLiteral {
                                generic_args.push(RbqType::Literal(source[arg_child.span()].trim().to_string()));
                            }
                        }
                    }
                }
                RbqElementType::FieldDef => {
                    if let Some(node) = child.as_node() {
                        inline_fields.push(RbqField::lower(node, source));
                    }
                }
                RbqElementType::Literal => {
                    literal = Some(source[child.span()].trim().to_string());
                }
                _ => {}
            }
        }

        if !inline_fields.is_empty() {
            let mut res = RbqType::InlineStruct(inline_fields);
            if is_ref {
                res = RbqType::PhysicalRef(Box::new(res));
            }
            if is_optional {
                res = RbqType::Optional(Box::new(res));
            }
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
            match child.kind::<RbqElementType>() {
                RbqElementType::Ident if name.is_empty() => name = source[child.span()].trim().to_string(),
                RbqElementType::AnnotationArgs => {
                    if let Some(node) = child.as_node() {
                        for arg_child in node.children() {
                            match arg_child.kind::<RbqElementType>() {
                                RbqElementType::Literal | RbqElementType::MagicVar | RbqElementType::BinaryExpr | RbqElementType::MemberExpr | RbqElementType::CallExpr | RbqElementType::Ident => args.push(source[arg_child.span()].trim().to_string()),
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
    pub args: Vec<RbqField>,
    pub return_type: Option<RbqType>,
    pub body: Option<Vec<RbqExpr>>,
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
        let mut body = None;

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
                        args.push(RbqField::lower(node, source));
                    }
                }
                RbqElementType::TypeRef => {
                    if let Some(node) = child.as_node() {
                        return_type = Some(RbqType::lower(node, source));
                    }
                }
                RbqElementType::Block => {
                    if let Some(node) = child.as_node() {
                        let mut expressions = Vec::new();
                        for block_child in node.children() {
                            if let RedTree::Node(expr_node) = block_child {
                                expressions.push(RbqExpr::lower(expr_node, source));
                            }
                        }
                        body = Some(expressions);
                    }
                }
                _ => {}
            }
        }

        Self { annotations, name, args, return_type, body, span }
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
    Closure { args: Vec<String>, body: Vec<RbqExpr> },
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
        let kind = match red.kind::<RbqElementType>() {
            RbqElementType::Literal => {
                let text = source[span.clone()].trim().to_string();
                if let Some(leaf) = red.children().find_map(|c| {
                    let t = c.as_token()?;
                    let k = t.kind();
                    match k {
                        RbqTokenType::StringLiteral | RbqTokenType::NumberLiteral | RbqTokenType::TrueKw | RbqTokenType::FalseKw => Some(t),
                        _ => None,
                    }
                }) {
                    match leaf.kind() {
                        RbqTokenType::StringLiteral => {
                            // Strip quotes
                            let s = if text.starts_with('"') && text.ends_with('"') {
                                text[1..text.len() - 1].to_string()
                            } else if text.starts_with('\'') && text.ends_with('\'') {
                                text[1..text.len() - 1].to_string()
                            } else {
                                text
                            };
                            RbqExprKind::Literal(RbqLiteral::String(s))
                        }
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
            RbqElementType::Ident => {
                let text = source[span.clone()].trim().to_string();
                RbqExprKind::Identifier(text)
            }
            RbqElementType::MagicVar => {
                let text = source[span.clone()].trim().to_string();
                RbqExprKind::MagicVar(text)
            }
            RbqElementType::BinaryExpr => {
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
                        RedTree::Leaf(leaf) => {
                            let k = leaf.kind();
                            if k == RbqTokenType::Plus
                                || k == RbqTokenType::Minus
                                || k == RbqTokenType::Star
                                || k == RbqTokenType::Slash
                                || k == RbqTokenType::EqEq
                                || k == RbqTokenType::NotEq
                                || k == RbqTokenType::Lt
                                || k == RbqTokenType::Gt
                                || k == RbqTokenType::LtEq
                                || k == RbqTokenType::GtEq
                                || k == RbqTokenType::AndAnd
                                || k == RbqTokenType::OrOr
                                || k == RbqTokenType::Eq
                            {
                                op = source[leaf.span()].trim().to_string()
                            }
                        }
                    }
                }
                if let (Some(left), Some(right)) = (left, right) { RbqExprKind::Binary { left, op, right } } else { RbqExprKind::Identifier(source[span.clone()].to_string()) }
            }
            RbqElementType::UnaryExpr => {
                let mut op = String::new();
                let mut expr = None;
                for child in red.children() {
                    match child {
                        RedTree::Node(node) => expr = Some(Box::new(RbqExpr::lower(node, source))),
                        RedTree::Leaf(leaf) => {
                            let k = leaf.kind();
                            if k == RbqTokenType::Not || k == RbqTokenType::Minus {
                                op = source[leaf.span()].trim().to_string()
                            }
                        }
                    }
                }
                if let Some(expr) = expr { RbqExprKind::Unary { op, expr } } else { RbqExprKind::Identifier(source[span.clone()].to_string()) }
            }
            RbqElementType::CallExpr => {
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
            RbqElementType::MemberExpr => {
                let mut object = None;
                let mut property = String::new();
                for child in red.children() {
                    match child {
                        RedTree::Node(node) => object = Some(Box::new(RbqExpr::lower(node, source))),
                        RedTree::Leaf(leaf) if leaf.kind() == RbqTokenType::Ident => property = source[leaf.span()].trim().to_string(),
                        _ => {}
                    }
                }
                if let Some(object) = object { RbqExprKind::Member { object, property } } else { RbqExprKind::Identifier(source[span.clone()].to_string()) }
            }
            RbqElementType::QueryPipeline => {
                let mut base = None;
                let mut steps = Vec::new();
                for child in red.children() {
                    match child {
                        RedTree::Node(node) => {
                            let k = node.kind::<RbqElementType>();
                            if k == RbqElementType::PipelineStep {
                                steps.push(RbqPipelineStep::lower(node, source))
                            }
                            else if base.is_none() {
                                // Ignore symbols like { and | that might be nodes if not careful
                                // But usually they are tokens.
                                base = Some(Box::new(RbqExpr::lower(node, source)))
                            }
                        }
                        _ => {}
                    }
                }
                if let Some(base) = base { RbqExprKind::Pipeline { base, steps } } else { RbqExprKind::Identifier(source[span.clone()].to_string()) }
            }
            RbqElementType::Closure => {
                let mut args = Vec::new();
                let mut body = Vec::new();
                for child in red.children() {
                    match child {
                        RedTree::Node(node) => match node.kind::<RbqElementType>() {
                            RbqElementType::ClosureArgs => {
                                for arg in node.children() {
                                    match arg {
                                        RedTree::Leaf(leaf) => {
                                            if leaf.kind() == RbqTokenType::Ident {
                                                args.push(source[leaf.span()].trim().to_string())
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            _ => {
                                body.push(RbqExpr::lower(node, source));
                            }
                        },
                        _ => {}
                    }
                }
                RbqExprKind::Closure { args, body }
            }
            RbqElementType::Block => {
                let mut expressions = Vec::new();
                for child in red.children() {
                    if let RedTree::Node(node) = child {
                        expressions.push(RbqExpr::lower(node, source));
                    }
                }
                RbqExprKind::Block(expressions)
            }
            RbqElementType::Expression => {
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
                RedTree::Leaf(leaf) if leaf.kind() == RbqTokenType::Ident && name.is_empty() => name = source[leaf.span()].trim().to_string(),
                _ => {}
            }
        }
        Self { name, args, span }
    }
}
