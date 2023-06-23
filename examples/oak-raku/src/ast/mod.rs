use oak_core::Range;

/// Root AST node for Raku.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RakuRoot {
    /// The span of the root node.
    pub span: Range<usize>,
}
