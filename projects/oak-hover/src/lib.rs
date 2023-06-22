#![feature(new_range_api)]
use core::range::Range;
use oak_core::{language::Language, tree::RedNode};
use serde::{Deserialize, Serialize};

/// Represents hover information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hover {
    /// The hover's content as a markdown string.
    pub contents: String,
    /// An optional span to which this hover applies.
    #[serde(with = "oak_core::serde_range::option", bound(serialize = "", deserialize = ""))]
    pub range: Option<Range<usize>>,
}

/// Trait for languages that support hover information.
pub trait HoverProvider<L: Language> {
    /// Returns hover information at the given range.
    fn hover(&self, root: &RedNode<L>, range: Range<usize>) -> Option<Hover>;
}
