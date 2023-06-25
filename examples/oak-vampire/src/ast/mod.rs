use crate::parser::VampireElementType;
use oak_core::tree::TypedNode;

/// Vampire root node.
#[derive(Debug, Clone)]
pub struct VampireRoot {
    pub span: oak_core::Range<usize>,
    pub formulas: Vec<()>,
}
