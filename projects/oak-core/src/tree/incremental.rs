use crate::tree::{
    green_tree::{GreenLeaf, GreenNode, GreenTree},
    red_tree::RedNode,
};
use alloc::{rc::Rc, string::String, vec::Vec};
use core::ops::Range;

/// Green tree builder (simplified) for constructing nodes from child elements.
///
/// This builder provides a fluent API for constructing green nodes in an
/// incremental parsing system.
#[derive(Debug, Clone)]
pub struct GreenBuilder<K: Copy> {
    children: Vec<GreenTree<K>>,
}

impl<K: Copy> GreenBuilder<K> {
    /// Creates a new empty green builder.
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = GreenBuilder::<K>::new();
    /// ```
    pub fn new() -> Self {
        Self { children: Vec::new() }
    }

    /// Adds a tokens leaf to the builder.
    ///
    /// # Arguments
    ///
    /// * `kind` - The tokens kind/type
    /// * `len` - The tokens length in bytes
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = GreenBuilder::new().tokens(K::Identifier, 5);
    /// ```
    pub fn token(mut self, kind: K, len: usize) -> Self {
        self.children.push(GreenTree::Leaf(GreenLeaf::new(kind, len)));
        self
    }

    /// Adds an existing green tree element to the builder.
    ///
    /// # Arguments
    ///
    /// * `elem` - The green tree element to add
    ///
    /// # Examples
    ///
    /// ```
    /// let child_node = some_green_tree;
    /// let builder = GreenBuilder::new().push(child_node);
    /// ```
    pub fn push(mut self, elem: GreenTree<K>) -> Self {
        self.children.push(elem);
        self
    }

    /// Finishes building and creates a new green node with the specified kind.
    ///
    /// # Arguments
    ///
    /// * `kind` - The node kind/type for the finished green node
    ///
    /// # Returns
    ///
    /// An `Rc<GreenNode<K>>` containing all the accumulated children.
    ///
    /// # Examples
    ///
    /// ```
    /// let green_node = GreenBuilder::new().token(K::Number, 3).finish(K::Expression);
    /// ```
    pub fn finish(self, kind: K) -> Rc<GreenNode<K>> {
        GreenNode::new(kind, self.children)
    }
}
