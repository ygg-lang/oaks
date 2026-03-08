use super::{
    common_nodes::RbqAnnotation,
    data_def_nodes::{RbqEnum, RbqField, RbqStruct, RbqUnion},
    expression_nodes::{RbqExpr, RbqExprKind},
    type_nodes::RbqType,
};
use crate::{language::RbqLanguage, parser::element_type::RbqElementType};
use oak_core::{Range, tree::RedNode};

/// Represents a namespace definition in RBQ.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RbqNamespace {
    /// Annotations applied to the namespace.
    pub annotations: Vec<RbqAnnotation>,
    /// The path of the namespace.
    pub path: String,
    /// Items contained within the namespace.
    pub items: Vec<super::super::RbqItem>,
    /// The source range of the namespace definition.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl RbqNamespace {
    /// Lowers a red node into an `RbqNamespace` AST node.
    pub fn lower(red: oak_core::tree::RedNode<RbqLanguage>, source: &str) -> Self {
        let span = red.span();
        let mut annotations = Vec::new();
        let mut path = String::new();
        let mut items = Vec::new();
        let mut pending_annotations = Vec::new();

        for child in red.children() {
            match child.kind::<RbqElementType>() {
                RbqElementType::Annotation => {
                    if let Some(node) = child.as_node() {
                        pending_annotations.push(RbqAnnotation::lower(node, source))
                    }
                }
                RbqElementType::Ident | RbqElementType::Dot => {
                    path.push_str(source[child.span()].trim());
                }
                RbqElementType::StructDef | RbqElementType::ClassDef => {
                    if let Some(node) = child.as_node() {
                        let mut s = RbqStruct::lower(node, source);
                        s.annotations.extend(pending_annotations.drain(..));
                        items.push(super::super::RbqItem::Struct(s))
                    }
                }
                RbqElementType::EnumDef => {
                    if let Some(node) = child.as_node() {
                        let mut e = RbqEnum::lower(node, source);
                        e.annotations.extend(pending_annotations.drain(..));
                        items.push(super::super::RbqItem::Enum(e))
                    }
                }
                RbqElementType::UnionDef => {
                    if let Some(node) = child.as_node() {
                        let mut u = RbqUnion::lower(node, source);
                        u.annotations.extend(pending_annotations.drain(..));
                        items.push(super::super::RbqItem::Union(u))
                    }
                }
                RbqElementType::TraitDef => {
                    if let Some(node) = child.as_node() {
                        let mut t = RbqTrait::lower(node, source);
                        t.annotations.extend(pending_annotations.drain(..));
                        items.push(super::super::RbqItem::Trait(t))
                    }
                }
                RbqElementType::MicroDef => {
                    if let Some(node) = child.as_node() {
                        let mut m = RbqMicro::lower(node, source);
                        m.annotations.extend(pending_annotations.drain(..));
                        items.push(super::super::RbqItem::Micro(m))
                    }
                }
                RbqElementType::ImportDef => {
                    if let Some(node) = child.as_node() {
                        let mut i = RbqImport::lower(node, source);
                        i.annotations.extend(pending_annotations.drain(..));
                        items.push(super::super::RbqItem::Import(i))
                    }
                }
                RbqElementType::TypeDef => {
                    if let Some(node) = child.as_node() {
                        let mut t = RbqTypeAlias::lower(node, source);
                        t.annotations.extend(pending_annotations.drain(..));
                        items.push(super::super::RbqItem::TypeAlias(t))
                    }
                }
                _ => {}
            }
        }

        Self { annotations, path, items, span }
    }
}

/// Represents a trait definition in RBQ.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RbqTrait {
    /// Annotations applied to the trait.
    pub annotations: Vec<RbqAnnotation>,
    /// The name of the trait.
    pub name: String,
    /// Items contained within the trait.
    pub items: Vec<RbqTraitItem>,
    /// The source range of the trait definition.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents an item within a trait definition in RBQ.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RbqTraitItem {
    /// A field item.
    Field(RbqField),
    /// A method item.
    Method(RbqMicro),
}

impl RbqTrait {
    /// Lowers a red node into an `RbqTrait` AST node.
    pub fn lower(red: RedNode<RbqLanguage>, source: &str) -> Self {
        let span = red.span();
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
                RbqElementType::Ident if name.is_empty() => {
                    name = source[child.span()].trim().to_string();
                }
                RbqElementType::FieldDef => {
                    if let Some(node) = child.as_node() {
                        let field = RbqField::lower(node, source);
                        // If it's a field with a Micro value, it's a method
                        if let Some(expr) = &field.default_value {
                            if let RbqExprKind::Identifier(id) = &expr.kind {
                                if id == "micro" {
                                    items.push(RbqTraitItem::Method(RbqMicro { annotations: field.annotations.clone(), name: field.name.clone(), args: Vec::new(), return_type: None, body: None, span: node.span() }));
                                    continue;
                                }
                            }
                            else if let RbqExprKind::Call { callee, args } = &expr.kind {
                                if let RbqExprKind::Identifier(id) = &callee.kind {
                                    if id == "micro" {
                                        let mut micro = RbqMicro { annotations: field.annotations.clone(), name: field.name.clone(), args: Vec::new(), return_type: None, body: None, span: node.span() };
                                        // Try to map CallExpr args to RbqField args
                                        for arg_expr in args {
                                            if let RbqExprKind::Binary { left, op, right } = &arg_expr.kind {
                                                if op == ":" {
                                                    if let RbqExprKind::Identifier(arg_name) = &left.kind {
                                                        let type_path = source[right.span.clone()].trim().to_string();
                                                        micro.args.push(RbqField {
                                                            annotations: Vec::new(),
                                                            name: arg_name.clone(),
                                                            type_ref: RbqType::Named { path: type_path, generic_args: Vec::new(), is_physical_ptr: false, is_optional: false, span: right.span.clone() },
                                                            default_value: None,
                                                            span: arg_expr.span.clone(),
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

        Self { annotations, name, items, span }
    }
}

/// Represents a micro (method/function) definition in RBQ.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RbqMicro {
    /// Annotations applied to the micro.
    pub annotations: Vec<RbqAnnotation>,
    /// The name of the micro.
    pub name: String,
    /// List of parameters for the micro.
    pub args: Vec<RbqField>,
    /// The return type of the micro.
    pub return_type: Option<RbqType>,
    /// The body of the micro, containing a list of expressions.
    pub body: Option<Vec<RbqExpr>>,
    /// The source range of the micro definition.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl RbqMicro {
    /// Lowers a red node into an `RbqMicro` AST node.
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
                            if let Some(expr_node) = block_child.as_node() {
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

/// Represents an import definition in RBQ.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RbqImport {
    /// Annotations applied to the import.
    pub annotations: Vec<RbqAnnotation>,
    /// The path of the import.
    pub path: String,
    /// The source range of the import definition.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl RbqImport {
    /// Lowers a red node into an `RbqImport` AST node.
    pub fn lower(red: RedNode<RbqLanguage>, source: &str) -> Self {
        let span = red.span();
        let mut annotations = Vec::new();
        let mut path = String::new();

        for child in red.children() {
            match child.kind::<RbqElementType>() {
                RbqElementType::Annotation => {
                    if let Some(node) = child.as_node() {
                        annotations.push(RbqAnnotation::lower(node, source))
                    }
                }
                RbqElementType::Ident | RbqElementType::Dot => {
                    path.push_str(source[child.span()].trim());
                }
                _ => {}
            }
        }

        Self { annotations, path, span }
    }
}

/// Represents a type alias definition in RBQ.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RbqTypeAlias {
    /// Annotations applied to the type alias.
    pub annotations: Vec<RbqAnnotation>,
    /// The name of the type alias.
    pub name: String,
    /// The type reference the alias points to.
    pub type_ref: RbqType,
    /// The source range of the type alias definition.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl RbqTypeAlias {
    /// Lowers a red node into an `RbqTypeAlias` AST node.
    pub fn lower(red: RedNode<RbqLanguage>, source: &str) -> Self {
        let span = red.span();
        let mut annotations = Vec::new();
        let mut name = String::new();
        let mut type_ref = RbqType::Named { path: "unknown".to_string(), generic_args: Vec::new(), is_physical_ptr: false, is_optional: false, span: span.clone() };

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

        Self { annotations, name, type_ref, span }
    }
}
