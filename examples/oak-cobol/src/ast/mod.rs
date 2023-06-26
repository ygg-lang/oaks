use crate::language::CobolLanguage;
use core::range::Range;
use oak_core::tree::{RedLeaf, RedNode};

/// A node in the COBOL red tree.
pub type CobolNode<'a> = RedNode<'a, CobolLanguage>;
/// A token in the COBOL red tree.
pub type CobolToken = RedLeaf<CobolLanguage>;

/// The root node of the COBOL AST.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CobolRoot {
    /// The source range of this root node.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}
