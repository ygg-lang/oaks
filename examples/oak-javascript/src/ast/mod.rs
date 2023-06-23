//! JavaScript AST nodes.

use core::range::Range;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// JavaScript root node.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct JavaScriptRoot {
    /// The span of the root node.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}
