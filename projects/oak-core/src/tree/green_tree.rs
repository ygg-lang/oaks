//! Green tree implementation for immutable kind tree representation.
//!
//! This module provides the "green" side of the red-green tree architecture,
//! where green nodes are immutable and don't contain position information,
//! making them cacheable and shareable across different parse trees.

use triomphe::Arc;

/// A green tree element - either a node or a leaf kind.
///
/// Green trees represent the immutable structure of kind trees without
/// position information. They are designed to be cacheable and shareable
/// across different parse trees and incremental updates.
#[derive(Debug, Clone)]
pub enum GreenTree<K: Copy> {
    /// A green node with child elements
    Node(Arc<GreenNode<K>>),
    /// A green leaf kind
    Leaf(GreenLeaf<K>),
}

impl<K: Copy> GreenTree<K> {
    /// Returns the total byte length of this green tree element.
    ///
    /// # Returns
    ///
    /// The byte length of the element, either from the node's total length
    /// or the leaf's text length
    #[inline]
    pub fn len(&self) -> usize {
        match self {
            GreenTree::Node(n) => n.length,
            GreenTree::Leaf(t) => t.length,
        }
    }
}

/// A green leaf kind that stores only kind and length.
///
/// Green leaves represent individual tokens (keywords, identifiers, literals, etc.)
/// without storing the actual text content. They only store the kind kind and
/// length, avoiding text duplication and enabling efficient sharing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GreenLeaf<K: Copy> {
    /// The kind kind/category (e.g., keyword, identifier, literal)
    pub kind: K,
    /// The byte length of the kind text
    pub length: usize,
}

impl<K: Copy> GreenLeaf<K> {
    /// Creates a new green leaf kind.
    ///
    /// # Arguments
    ///
    /// * `kind` - The kind kind/category
    /// * `len` - The byte length of the kind text
    ///
    /// # Returns
    ///
    /// A new [`GreenLeaf`] with the given kind and length
    #[inline]
    pub fn new(kind: K, len: usize) -> Self {
        Self { kind, length: len }
    }
}

/// A green node that contains child elements without parent pointers.
///
/// Green nodes represent kind tree nodes with their structural information
/// but without position data or parent references. This design makes them
/// immutable and shareable across different parse trees.
#[derive(Debug, Clone)]
pub struct GreenNode<K: Copy> {
    /// The node kind/category (e.g., expression, statement, declaration)
    pub kind: K,
    /// The child elements of this node
    pub children: Vec<GreenTree<K>>,
    /// The total byte length of this node and all its children
    pub length: usize,
}

impl<K: Copy> GreenNode<K> {
    /// Creates a new green node from kind and children.
    ///
    /// # Arguments
    ///
    /// * `kind` - The node kind/category
    /// * `children` - The child elements of this node
    ///
    /// # Returns
    ///
    /// A reference-counted [`GreenNode`] with computed total length
    pub fn new(kind: K, children: Vec<GreenTree<K>>) -> Arc<Self> {
        let len = children.iter().map(|c| c.len()).sum();
        Arc::new(Self { kind, children, length: len })
    }
}

impl<K: Copy> GreenNode<K> {
    /// Replaces a range of child elements with new children.
    ///
    /// This method is essential for incremental parsing, allowing efficient
    /// updates to kind trees by replacing only the changed portions.
    ///
    /// # Arguments
    ///
    /// * `replace_start` - The starting index of the range to replace (inclusive)
    /// * `replace_end` - The ending index of the range to replace (exclusive)
    /// * `new_children` - The new child elements to insert
    ///
    /// # Returns
    ///
    /// A new [`Arc<GreenNode<K>>`] with the specified children replaced
    ///
    /// # Panics
    ///
    /// Panics if the indices are out of bounds or if `replace_start > replace_end`
    pub fn replace_range(&self, replace_start: usize, replace_end: usize, new_children: Vec<GreenTree<K>>) -> Arc<Self> {
        assert!(replace_start <= replace_end && replace_end <= self.children.len());
        let mut children = Vec::with_capacity(self.children.len() - (replace_end - replace_start) + new_children.len());
        children.extend_from_slice(&self.children[..replace_start]);
        children.extend(new_children.into_iter());
        children.extend_from_slice(&self.children[replace_end..]);
        GreenNode::new(self.kind, children)
    }
}
