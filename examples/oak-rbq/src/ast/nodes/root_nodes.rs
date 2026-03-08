use super::{
    common_nodes::RbqAnnotation,
    data_def_nodes::{RbqEnum, RbqStruct, RbqUnion},
    logic_def_nodes::{RbqImport, RbqMicro, RbqNamespace, RbqTrait, RbqTypeAlias},
};
use crate::{language::RbqLanguage, parser::element_type::RbqElementType};
use oak_core::{Range, tree::RedNode};

/// Represents the root of an RBQ AST.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RbqRoot {
    /// Items contained within the RBQ file.
    pub items: Vec<RbqItem>,
    /// The source range of the entire RBQ file.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents an item in an RBQ file.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RbqItem {
    /// A namespace definition.
    Namespace(RbqNamespace),
    /// A struct definition.
    Struct(RbqStruct),
    /// An enum definition.
    Enum(RbqEnum),
    /// A union definition.
    Union(RbqUnion),
    /// A trait definition.
    Trait(RbqTrait),
    /// A micro definition.
    Micro(RbqMicro),
    /// An import definition.
    Import(RbqImport),
    /// A type alias definition.
    TypeAlias(RbqTypeAlias),
}

impl RbqRoot {
    /// Lowers a red node into an `RbqRoot` AST node.
    pub fn lower(red: oak_core::tree::RedNode<RbqLanguage>, source: &str) -> Self {
        let span = red.span();
        let mut items = Vec::new();
        let mut pending_annotations = Vec::new();

        for child in red.children() {
            match child.kind::<RbqElementType>() {
                RbqElementType::Annotation => {
                    if let Some(node) = child.as_node() {
                        pending_annotations.push(RbqAnnotation::lower(node, source))
                    }
                }
                RbqElementType::NamespaceDef => {
                    if let Some(node) = child.as_node() {
                        let mut ns = RbqNamespace::lower(node, source);
                        ns.annotations.extend(pending_annotations.drain(..));
                        items.push(RbqItem::Namespace(ns))
                    }
                }
                RbqElementType::StructDef | RbqElementType::ClassDef => {
                    if let Some(node) = child.as_node() {
                        let mut s = RbqStruct::lower(node, source);
                        s.annotations.extend(pending_annotations.drain(..));
                        items.push(RbqItem::Struct(s))
                    }
                }
                RbqElementType::EnumDef => {
                    if let Some(node) = child.as_node() {
                        let mut e = RbqEnum::lower(node, source);
                        e.annotations.extend(pending_annotations.drain(..));
                        items.push(RbqItem::Enum(e))
                    }
                }
                RbqElementType::UnionDef => {
                    if let Some(node) = child.as_node() {
                        let mut u = RbqUnion::lower(node, source);
                        u.annotations.extend(pending_annotations.drain(..));
                        items.push(RbqItem::Union(u))
                    }
                }
                RbqElementType::TraitDef => {
                    if let Some(node) = child.as_node() {
                        let mut t = RbqTrait::lower(node, source);
                        t.annotations.extend(pending_annotations.drain(..));
                        items.push(RbqItem::Trait(t))
                    }
                }
                RbqElementType::MicroDef => {
                    if let Some(node) = child.as_node() {
                        let mut m = RbqMicro::lower(node, source);
                        m.annotations.extend(pending_annotations.drain(..));
                        items.push(RbqItem::Micro(m))
                    }
                }
                RbqElementType::ImportDef => {
                    if let Some(node) = child.as_node() {
                        let mut i = RbqImport::lower(node, source);
                        i.annotations.extend(pending_annotations.drain(..));
                        items.push(RbqItem::Import(i))
                    }
                }
                RbqElementType::TypeDef => {
                    if let Some(node) = child.as_node() {
                        let mut t = RbqTypeAlias::lower(node, source);
                        t.annotations.extend(pending_annotations.drain(..));
                        items.push(RbqItem::TypeAlias(t))
                    }
                }
                _ => {}
            }
        }

        Self { items, span }
    }
}
