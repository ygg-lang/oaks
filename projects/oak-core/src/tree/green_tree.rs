//! Green tree implementation for immutable kind tree representation.
//!
//! This module provides the "green" side of the red-green tree architecture.
//! In this high-performance implementation, green nodes are allocated in a
//! `SyntaxArena` and do not use reference counting.

use crate::Language;
use std::{
    fmt,
    hash::{Hash, Hasher},
};

/// A green tree element - either a node or a leaf kind.
///
/// Green trees represent the immutable structure of kind trees without
/// position information.
///
/// # Design Note: Lifetimes and References
/// We use `&'a GreenNode<'a, L>` instead of an owned `GreenNode<'a, L>` for two reasons:
/// 1. **Recursion**: `GreenTree` is a recursive structure. To avoid infinite size, we must use a pointer.
///    Since green nodes are allocated in an arena, a reference `&'a` is the most efficient choice.
/// 2. **Handles**: Holding a reference (8 bytes) is much cheaper than holding the whole 
///    `GreenNode` struct (24+ bytes), making handles like `RedNode` very lightweight.
pub enum GreenTree<'a, L: Language> {
    /// A green node with child elements
    Node(&'a GreenNode<'a, L>),
    /// A green leaf kind
    Leaf(GreenLeaf<L>),
}

// Manually implement Clone/Copy to avoid L: Copy bound
impl<'a, L: Language> Clone for GreenTree<'a, L> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, L: Language> Copy for GreenTree<'a, L> {}

impl<'a, L: Language> fmt::Debug for GreenTree<'a, L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Node(node) => fmt::Debug::fmt(node, f),
            Self::Leaf(leaf) => fmt::Debug::fmt(leaf, f),
        }
    }
}

impl<'a, L: Language> PartialEq for GreenTree<'a, L> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Node(l0), Self::Node(r0)) => l0 == r0,
            (Self::Leaf(l0), Self::Leaf(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl<'a, L: Language> Eq for GreenTree<'a, L> {}

impl<'a, L: Language> Hash for GreenTree<'a, L> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Node(node) => node.hash(state),
            Self::Leaf(leaf) => leaf.hash(state),
        }
    }
}

impl<'a, L: Language> GreenTree<'a, L> {
    /// Returns the total byte length of this green tree element.
    ///
    /// For nodes, this is the sum of all children's lengths.
    /// For leaves, this is the length of the token.
    #[inline]
    pub fn len(&self) -> u32 {
        match self {
            GreenTree::Node(n) => n.byte_length,
            GreenTree::Leaf(t) => t.length,
        }
    }

    /// Checks if this green tree element is a node.
    #[inline]
    pub fn is_node(&self) -> bool {
        matches!(self, Self::Node(_))
    }

    /// Checks if this green tree element is a leaf.
    #[inline]
    pub fn is_leaf(&self) -> bool {
        matches!(self, Self::Leaf(_))
    }

    /// Returns the element as a node if it is one.
    #[inline]
    pub fn as_node(&self) -> Option<&'a GreenNode<'a, L>> {
        match self {
            Self::Node(n) => Some(n),
            _ => None,
        }
    }

    /// Returns the element as a leaf if it is one.
    #[inline]
    pub fn as_leaf(&self) -> Option<GreenLeaf<L>> {
        match self {
            Self::Leaf(l) => Some(*l),
            _ => None,
        }
    }
}

/// A green leaf kind that stores only kind and length.
///
/// Leaves are the terminal elements of the green tree, representing
/// individual tokens in the source code.
pub struct GreenLeaf<L: Language> {
    /// The token kind/category.
    pub kind: L::TokenType,
    /// The byte length of the token text.
    pub length: u32,
    /// Optional index into the metadata store for provenance information.
    ///
    /// This is used to track where a token came from in complex
    /// transformations or multi-file sources.
    pub metadata: Option<std::num::NonZeroU32>,
}

impl<L: Language> fmt::Debug for GreenLeaf<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GreenLeaf").field("kind", &self.kind).field("length", &self.length).field("metadata", &self.metadata).finish()
    }
}

impl<L: Language> Hash for GreenLeaf<L> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.kind.hash(state);
        self.length.hash(state);
        self.metadata.hash(state)
    }
}

// Manually implement Clone/Copy to avoid L: Copy bound
impl<L: Language> Clone for GreenLeaf<L> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<L: Language> Copy for GreenLeaf<L> {}

impl<L: Language> PartialEq for GreenLeaf<L> {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.length == other.length && self.metadata == other.metadata
    }
}

impl<L: Language> Eq for GreenLeaf<L> {}

impl<L: Language> GreenLeaf<L> {
    /// Creates a new green leaf kind.
    ///
    /// # Arguments
    ///
    /// * `kind` - The token type.
    /// * `len` - The length of the token in bytes.
    #[inline]
    pub fn new(kind: L::TokenType, len: u32) -> Self {
        Self { kind, length: len, metadata: None }
    }

    /// Creates a new green leaf kind with provenance metadata.
    ///
    /// # Arguments
    ///
    /// * `kind` - The token type.
    /// * `len` - The length of the token in bytes.
    /// * `metadata` - The metadata index.
    #[inline]
    pub fn with_metadata(kind: L::TokenType, len: u32, metadata: Option<std::num::NonZeroU32>) -> Self {
        Self { kind, length: len, metadata }
    }

    /// Returns the kind of this leaf.
    #[inline]
    pub fn kind(&self) -> L::TokenType {
        self.kind
    }

    /// Returns the length of this leaf in bytes.
    #[inline]
    pub fn length(&self) -> u32 {
        self.length
    }
}

/// A green node that contains child elements.
///
/// Green nodes are allocated in a `SyntaxArena` and hold a slice reference
/// to their children. They are POD (Plain Old Data) and strictly immutable.
///
/// Unlike red nodes, green nodes do not know their absolute position in the
/// source code, which makes them highly reusable for incremental parsing.
pub struct GreenNode<'a, L: Language> {
    /// The element type (kind) of this node.
    pub kind: L::ElementType,
    /// The children of this node, which can be other nodes or leaf tokens.
    pub children: &'a [GreenTree<'a, L>],
    /// The total text length of this node (sum of children's lengths) in bytes.
    pub byte_length: u32,
}

// Manually implement Clone to avoid L: Clone bound (though L usually is Clone)
impl<'a, L: Language> Clone for GreenNode<'a, L> {
    fn clone(&self) -> Self {
        Self { kind: self.kind, byte_length: self.byte_length, children: self.children }
    }
}

impl<'a, L: Language> PartialEq for GreenNode<'a, L> {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.byte_length == other.byte_length && self.children == other.children
    }
}

impl<'a, L: Language> Eq for GreenNode<'a, L> {}

impl<'a, L: Language> fmt::Debug for GreenNode<'a, L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GreenNode").field("kind", &self.kind).field("children", &self.children).field("length", &self.byte_length).finish()
    }
}

impl<'a, L: Language> Hash for GreenNode<'a, L> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.kind.hash(state);
        self.children.hash(state)
    }
}

impl<'a, L: Language> GreenNode<'a, L> {
    /// Creates a new green node from child elements.
    ///
    /// This function assumes the children slice is already allocated in the arena.
    /// It automatically calculates the total `text_len` by summing child lengths.
    ///
    /// # Arguments
    ///
    /// * `kind` - The node type.
    /// * `children` - The slice of child elements.
    pub fn new(kind: L::ElementType, children: &'a [GreenTree<'a, L>]) -> Self {
        let len: u32 = children.iter().map(|c| c.len()).sum();
        Self { kind, byte_length: len, children }
    }

    /// Returns the kind of this node.
    #[inline]
    pub fn kind(&self) -> L::ElementType {
        self.kind
    }

    /// Returns the total text length of this node in bytes.
    #[inline]
    pub fn text_len(&self) -> u32 {
        self.byte_length
    }

    /// Returns the children of this node.
    #[inline]
    pub fn children(&self) -> &'a [GreenTree<'a, L>] {
        self.children
    }

    /// Returns a specific child at index.
    ///
    /// Returns `None` if the index is out of bounds.
    #[inline]
    pub fn child_at(&self, index: usize) -> Option<&'a GreenTree<'a, L>> {
        self.children.get(index)
    }

    /// Checks if this node has any children.
    #[inline]
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    /// Returns the number of children in this node.
    #[inline]
    pub fn children_count(&self) -> usize {
        self.children.len()
    }
}
