use crate::parser::VampireElementType;
use oak_core::tree::TypedNode;

/// Vampire root node.
#[derive(Debug, Clone)]
pub struct VampireRoot {
    /// The source span covered by this root node.
    pub span: oak_core::Range<usize>,
    /// The collection of formulas in the Vampire document.
    pub formulas: Vec<()>,
}
