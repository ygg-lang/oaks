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
    #[inline]
    pub fn len(&self) -> u32 {
        match self {
            GreenTree::Node(n) => n.text_len,
            GreenTree::Leaf(t) => t.length,
        }
    }
}

/// A green leaf kind that stores only kind and length.
pub struct GreenLeaf<L: Language> {
    /// The kind kind/category
    pub kind: L::TokenType,
    /// The byte length of the kind text
    pub length: u32,
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
        self.kind == other.kind && self.length == other.length
    }
}

impl<L: Language> Eq for GreenLeaf<L> {}

impl<L: Language> Hash for GreenLeaf<L> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.kind.hash(state);
        self.length.hash(state);
    }
}

impl<L: Language> fmt::Debug for GreenLeaf<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GreenLeaf").field("kind", &self.kind).field("length", &self.length).finish()
    }
}

impl<L: Language> GreenLeaf<L> {
    /// Creates a new green leaf kind.
    #[inline]
    pub fn new(kind: L::TokenType, len: u32) -> Self {
        Self { kind, length: len }
    }
}

/// A green node that contains child elements.
///
/// Green nodes are allocated in a `SyntaxArena` and hold a slice reference
/// to their children. They are POD (Plain Old Data) and strictly immutable.
pub struct GreenNode<'a, L: Language> {
    /// The element type (kind) of this node.
    pub kind: L::ElementType,
    /// The total text length of this node (sum of children's lengths).
    pub text_len: u32,
    /// The children of this node.
    pub children: &'a [GreenTree<'a, L>],
}

// Manually implement Clone to avoid L: Clone bound (though L usually is Clone)
impl<'a, L: Language> Clone for GreenNode<'a, L> {
    fn clone(&self) -> Self {
        Self { kind: self.kind, text_len: self.text_len, children: self.children }
    }
}

impl<'a, L: Language> PartialEq for GreenNode<'a, L> {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.text_len == other.text_len && self.children == other.children
    }
}

impl<'a, L: Language> Eq for GreenNode<'a, L> {}

impl<'a, L: Language> fmt::Debug for GreenNode<'a, L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GreenNode").field("kind", &self.kind).field("children", &self.children).field("length", &self.text_len).finish()
    }
}

impl<'a, L: Language> Hash for GreenNode<'a, L> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.kind.hash(state);
        self.children.hash(state);
    }
}

impl<'a, L: Language> GreenNode<'a, L> {
    /// Creates a new green node from child elements.
    ///
    /// This function assumes the children slice is already allocated in the arena.
    pub fn new(kind: L::ElementType, children: &'a [GreenTree<'a, L>]) -> Self {
        let len: u32 = children.iter().map(|c| c.len()).sum();
        Self { kind, text_len: len, children }
    }

    /// Returns the kind of this node.
    #[inline]
    pub fn kind(&self) -> L::ElementType {
        self.kind
    }

    /// Returns the total text length of this node.
    #[inline]
    pub fn text_len(&self) -> u32 {
        self.text_len
    }

    /// Returns the children of this node.
    #[inline]
    pub fn children(&self) -> &'a [GreenTree<'a, L>] {
        self.children
    }

    /// Returns a specific child at index.
    #[inline]
    pub fn child_at(&self, index: usize) -> Option<&'a GreenTree<'a, L>> {
        self.children.get(index)
    }
}
