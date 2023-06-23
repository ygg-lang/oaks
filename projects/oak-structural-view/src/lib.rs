#![feature(new_range_api)]
#![warn(missing_docs)]
//! Document structural view (结构化视图) support for the Oak language framework.
//!
//! This crate provides traits and structures for representing the hierarchical
//! structural view of a document, such as for an outline view or breadcrumbs.
use core::range::Range;
use oak_core::{
    language::{Language, UniversalElementRole},
    tree::RedNode,
};
use serde::{Deserialize, Serialize};

/// Represents an item in the document structure (e.g., in an outline or breadcrumbs).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureItem {
    /// The name of this item (e.g., function name, class name).
    pub name: String,
    /// More detail about this item (e.g., function signature, type).
    pub detail: Option<String>,
    /// The universal role of this element.
    pub role: UniversalElementRole,
    /// The range of the entire element in the source code.
    #[serde(with = "oak_core::serde_range", bound(serialize = "", deserialize = ""))]
    pub range: Range<usize>,
    /// The range that should be selected when clicking on this item.
    /// Usually the range of the identifier.
    #[serde(with = "oak_core::serde_range", bound(serialize = "", deserialize = ""))]
    pub selection_range: Range<usize>,
    /// Whether this item is deprecated.
    pub deprecated: bool,
    /// Nested structure items (e.g., methods within a class).
    pub children: Vec<StructureItem>,
}

/// Trait for languages that support structure view and navigation.
///
/// Benchmarked against IntelliJ's Structure View and LSP's `textDocument/documentSymbol`.
pub trait StructureProvider<L: Language> {
    /// Returns the hierarchical structure of the document.
    fn structure(&self, root: &RedNode<L>) -> Vec<StructureItem>;
}
