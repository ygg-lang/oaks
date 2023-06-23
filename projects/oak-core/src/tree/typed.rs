use crate::{
    Language,
    tree::{GreenNode, RedNode},
};

/// A trait for typed AST nodes that wrap a red node.
pub trait TypedNode<'a>: Sized {
    /// The language this node belongs to.
    type Language: Language;

    /// Attempts to cast a red node to this typed node.
    fn cast(node: RedNode<'a, Self::Language>) -> Option<Self>;

    /// Returns the underlying green node.
    fn green(&self) -> &GreenNode<'a, Self::Language>;
}
