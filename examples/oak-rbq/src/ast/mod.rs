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
                    RbqTokenType::NamespaceDef => items.push(RbqItem::Namespace(RbqNamespace::lower(node, source))),
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
                    RbqTokenType::MicroDef => items.push(RbqItem::Micro(RbqMicro::lower(node, source))),
                    RbqTokenType::QueryPipeline | RbqTokenType::Expression => items.push(RbqItem::Query(RbqExpr::lower(node, source))),
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
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RbqStruct {
    pub annotations: Vec<RbqAnnotation>,
    pub name: String,
    pub using: Vec<String>,
    pub fields: Vec<RbqField>,
}

impl RbqStruct {
    pub fn lower(red: RedNode<RbqLanguage>, source: &str) -> Self {
        let mut annotations = Vec::new();
        let mut name = String::new();
        let mut using = Vec::new();
        let mut fields = Vec::new();

        for child in red.children() {
            match child {
                RedTree::Node(node) => match node.kind::<RbqTokenType>() {
                    RbqTokenType::Annotation => annotations.push(RbqAnnotation::lower(node, source)),
                    RbqTokenType::UsingDef => {
                        if let Some(path) = node.children().find(|c| c.kind::<RbqTokenType>() == RbqTokenType::Ident || c.kind::<RbqTokenType>() == RbqTokenType::Utf8Kw) {
                            using.push(source[path.span()].to_string())
                        }
                    }
                    RbqTokenType::FieldDef => fields.push(RbqField::lower(node, source)),
                    _ => {}
                },
                RedTree::Leaf(leaf) => {
                    if leaf.kind == RbqTokenType::Ident && name.is_empty() {
                        name = source[leaf.span].trim().to_string()
                    }
                }
            }
        }

        Self { annotations, name, using, fields }
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

        for child in red.children() {
            match child.kind::<RbqTokenType>() {
                RbqTokenType::Ident if name.is_empty() => name = source[child.span()].trim().to_string(),
                RbqTokenType::StructDef | RbqTokenType::ClassDef => {
                    if let Some(node) = child.as_node() {
                        items.push(RbqItem::Struct(RbqStruct::lower(node, source)))
                    }
                }
                RbqTokenType::UnionDef => {
                    if let Some(node) = child.as_node() {
                        items.push(RbqItem::Union(RbqUnion::lower(node, source)))
                    }
                }
                RbqTokenType::EnumDef => {
                    if let Some(node) = child.as_node() {
                        items.push(RbqItem::Enum(RbqEnum::lower(node, source)))
                    }
                }
                RbqTokenType::TraitDef => {
                    if let Some(node) = child.as_node() {
                        items.push(RbqItem::Trait(RbqTrait::lower(node, source)))
                    }
                }
                RbqTokenType::TypeDef => {
                    if let Some(node) = child.as_node() {
                        items.push(RbqItem::TypeAlias(RbqTypeAlias::lower(node, source)))
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
                    _ => {}
                },
                RedTree::Leaf(leaf) => {
                    if leaf.kind == RbqTokenType::Ident && name.is_empty() {
                        name = source[leaf.span].trim().to_string()
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
    pub variants: Vec<RbqEnumMember>,
}

impl RbqEnum {
    pub fn lower(red: RedNode<RbqLanguage>, source: &str) -> Self {
        let mut annotations = Vec::new();
        let mut name = String::new();
        let mut variants = Vec::new();

        for child in red.children() {
            match child {
                RedTree::Node(node) => match node.kind::<RbqTokenType>() {
                    RbqTokenType::Annotation => annotations.push(RbqAnnotation::lower(node, source)),
                    RbqTokenType::EnumMember => variants.push(RbqEnumMember::lower(node, source)),
                    _ => {}
                },
                RedTree::Leaf(leaf) => {
                    if leaf.kind == RbqTokenType::Ident && name.is_empty() {
                        name = source[leaf.span].trim().to_string()
                    }
                }
            }
        }

        Self { annotations, name, variants }
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
            match child.kind::<RbqTokenType>() {
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
    Tuple(Vec<String>),
    Struct(Vec<RbqField>),
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RbqTrait {
    pub annotations: Vec<RbqAnnotation>,
    pub name: String,
    pub items: Vec<RbqItem>,
}

impl RbqTrait {
    pub fn lower(red: RedNode<RbqLanguage>, source: &str) -> Self {
        let mut annotations = Vec::new();
        let mut name = String::new();
        let items = Vec::new();

        for child in red.children() {
            match child.kind::<RbqTokenType>() {
                RbqTokenType::Annotation => {
                    if let Some(node) = child.as_node() {
                        annotations.push(RbqAnnotation::lower(node, source))
                    }
                }
                RbqTokenType::Ident if name.is_empty() => name = source[child.span()].trim().to_string(),
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
    pub type_ref: String,
    pub default_value: Option<String>,
}

impl RbqField {
    pub fn lower(red: RedNode<RbqLanguage>, source: &str) -> Self {
        let mut annotations = Vec::new();
        let mut name = String::new();
        let mut type_ref = String::new();
        let mut default_value = None;

        for child in red.children() {
            match child.kind::<RbqTokenType>() {
                RbqTokenType::Annotation => {
                    if let Some(node) = child.as_node() {
                        annotations.push(RbqAnnotation::lower(node, source))
                    }
                }
                RbqTokenType::Ident if name.is_empty() => name = source[child.span()].trim().to_string(),
                RbqTokenType::TypeRef => type_ref = source[child.span()].trim().to_string(),
                RbqTokenType::Literal => default_value = Some(source[child.span()].trim().to_string()),
                _ => {}
            }
        }

        Self { annotations, name, type_ref, default_value }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RbqTypeAlias {
    pub annotations: Vec<RbqAnnotation>,
    pub name: String,
    pub type_ref: String,
}

impl RbqTypeAlias {
    pub fn lower(red: RedNode<RbqLanguage>, source: &str) -> Self {
        let mut annotations = Vec::new();
        let mut name = String::new();
        let mut type_ref = String::new();

        for child in red.children() {
            match child.kind::<RbqTokenType>() {
                RbqTokenType::Annotation => {
                    if let Some(node) = child.as_node() {
                        annotations.push(RbqAnnotation::lower(node, source))
                    }
                }
                RbqTokenType::Ident if name.is_empty() => name = source[child.span()].trim().to_string(),
                RbqTokenType::TypeRef => type_ref = source[child.span()].trim().to_string(),
                _ => {}
            }
        }

        Self { annotations, name, type_ref }
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
    pub name: String,
    pub args: Vec<(String, String)>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl RbqMicro {
    pub fn lower(red: RedNode<RbqLanguage>, source: &str) -> Self {
        let mut name = String::new();
        let mut args = Vec::new();

        let children: Vec<_> = red.children().collect();
        for i in 0..children.len() {
            let child = &children[i];
            match child.kind::<RbqTokenType>() {
                RbqTokenType::Ident if name.is_empty() => name = source[child.span()].trim().to_string(),
                RbqTokenType::TypeRef => {
                    if i >= 2 {
                        let prev_sibling = &children[i - 1];
                        if prev_sibling.kind::<RbqTokenType>() == RbqTokenType::Colon {
                            let arg_name_node = &children[i - 2];
                            if arg_name_node.kind::<RbqTokenType>() == RbqTokenType::Ident {
                                args.push((source[arg_name_node.span()].trim().to_string(), source[child.span()].trim().to_string()))
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        Self { name, args, span: red.span() }
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
    Literal(String),
    Identifier(String),
    Binary { left: Box<RbqExpr>, op: String, right: Box<RbqExpr> },
    Unary { op: String, expr: Box<RbqExpr> },
    Call { callee: Box<RbqExpr>, args: Vec<RbqExpr> },
    Member { object: Box<RbqExpr>, property: String },
    Pipeline { base: Box<RbqExpr>, steps: Vec<RbqPipelineStep> },
    Closure { args: Vec<String>, body: Box<RbqExpr> },
    MagicVar(String), // $, $key, $group
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
        let kind = match red.kind::<RbqTokenType>() {
            RbqTokenType::Literal => {
                let text = source[span.clone()].trim().to_string();
                RbqExprKind::Literal(text)
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
                        RedTree::Leaf(leaf) => {
                            let k = leaf.kind;
                            if TokenType::role(&k) == oak_core::UniversalTokenRole::Operator {
                                op = source[leaf.span].trim().to_string()
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
                        RedTree::Leaf(leaf) => {
                            let k = leaf.kind;
                            if TokenType::role(&k) == oak_core::UniversalTokenRole::Operator {
                                op = source[leaf.span].trim().to_string()
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
                        RedTree::Leaf(leaf) if leaf.kind == RbqTokenType::Ident => property = source[leaf.span].to_string(),
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
                                    if let RedTree::Leaf(leaf) = arg {
                                        if leaf.kind == RbqTokenType::Ident {
                                            args.push(source[leaf.span].to_string())
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
                RedTree::Leaf(leaf) if leaf.kind == RbqTokenType::Ident && name.is_empty() => name = source[leaf.span].to_string(),
                _ => {}
            }
        }
        Self { name, args, span }
    }
}
