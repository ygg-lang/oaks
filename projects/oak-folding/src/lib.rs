#![feature(new_range_api)]
#![warn(missing_docs)]
//! Code folding support for the Oak language framework.
//!
//! This crate provides traits and structures for identifying collapsible
//! regions in source code, such as functions, comments, or imports.
use core::range::Range;
use oak_core::{language::Language, tree::RedNode};
use serde::{Deserialize, Serialize};

/// Enum of folding range kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FoldingRangeKind {
    /// A comment block.
    Comment,
    /// An import section.
    Imports,
    /// A custom defined region.
    Region,
}

/// Represents a folding range in a document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoldingRange {
    /// The span of the range to fold.
    #[serde(with = "oak_core::serde_range", bound(serialize = "", deserialize = ""))]
    pub range: Range<usize>,
    /// The kind of folding range (e.g., 'comment', 'imports').
    pub kind: Option<FoldingRangeKind>,
}

/// Trait for languages that support code folding.
pub trait FoldingProvider<L: Language> {
    /// Returns all folding ranges for the given document.
    fn folding_ranges(&self, root: &RedNode<L>) -> Vec<FoldingRange> {
        let _ = root;
        Vec::new()
    }
}
