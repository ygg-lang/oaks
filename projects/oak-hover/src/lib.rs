#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc = include_str!("readme.md")]

use core::range::Range;
use oak_core::{language::Language, tree::RedNode};

/// Represents hover information.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hover {
    /// The hover's content as a markdown string.
    pub contents: String,
    /// An optional span to which this hover applies.
    #[cfg_attr(feature = "serde", serde(with = "serde_range_opt", bound(serialize = "", deserialize = "")))]
    pub range: Option<Range<usize>>,
}

#[cfg(feature = "serde")]
mod serde_range_opt {
    use super::*;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &Option<Range<usize>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(range) => oak_core::serde_range::serialize(range, serializer),
            None => serializer.serialize_none(),
        }
    }

    #[derive(Deserialize)]
    struct RangeDef {
        start: usize,
        end: usize,
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Range<usize>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt: Option<RangeDef> = Option::deserialize(deserializer)?;
        Ok(opt.map(|def| Range { start: def.start, end: def.end }))
    }
}

/// Trait for languages that support hover information.
pub trait HoverProvider<L: Language> {
    /// Returns hover information at the given range.
    fn hover(&self, root: &RedNode<L>, range: Range<usize>) -> Option<Hover>;
}
