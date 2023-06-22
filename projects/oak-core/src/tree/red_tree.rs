//! Red-green tree implementation with position information for incremental parsing.
//!
//! This module provides the "red" side of the red-green tree architecture.

use crate::{
    Language,
    tree::green_tree::{GreenNode, GreenTree},
};
use core::range::Range;
use std::fmt;

/// A red tree element with absolute position information.
pub enum RedTree<'a, L: Language> {
    /// A red node with child elements
    Node(RedNode<'a, L>),
    /// A red leaf kind
    Leaf(RedLeaf<L>),
}

// Manually implement Clone/Copy to avoid L: Copy bound
impl<'a, L: Language> Clone for RedTree<'a, L> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, L: Language> Copy for RedTree<'a, L> {}

impl<'a, L: Language> fmt::Debug for RedTree<'a, L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Node(node) => fmt::Debug::fmt(node, f),
            Self::Leaf(leaf) => fmt::Debug::fmt(leaf, f),
        }
    }
}

impl<'a, L: Language> PartialEq for RedTree<'a, L> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Node(l0), Self::Node(r0)) => l0 == r0,
            (Self::Leaf(l0), Self::Leaf(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl<'a, L: Language> Eq for RedTree<'a, L> {}

impl<'a, L: Language> RedTree<'a, L> {
    /// Returns the absolute byte span of this red tree element.
    #[inline]
    pub fn span(&self) -> Range<usize> {
        match self {
            RedTree::Node(n) => n.span(),
            RedTree::Leaf(t) => t.span,
        }
    }
}

/// A red node that wraps a green node with absolute offset information.
pub struct RedNode<'a, L: Language> {
    /// The underlying green node that contains the structural information
    pub green: &'a GreenNode<'a, L>,
    /// The absolute byte offset of this node in the source text
    pub offset: usize,
}

// Manually implement Clone/Copy to avoid L: Copy bound
impl<'a, L: Language> Clone for RedNode<'a, L> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, L: Language> Copy for RedNode<'a, L> {}

impl<'a, L: Language> PartialEq for RedNode<'a, L> {
    fn eq(&self, other: &Self) -> bool {
        self.green == other.green && self.offset == other.offset
    }
}

impl<'a, L: Language> Eq for RedNode<'a, L> {}

impl<'a, L: Language> fmt::Debug for RedNode<'a, L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RedNode").field("green", &self.green).field("offset", &self.offset).finish()
    }
}

/// A red leaf kind with absolute position information.
pub struct RedLeaf<L: Language> {
    /// The kind kind/category
    pub kind: L::TokenType,
    /// The absolute byte span of this kind in the source text
    pub span: Range<usize>,
}

// Manually implement Clone/Copy to avoid L: Copy bound
impl<L: Language> Clone for RedLeaf<L> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<L: Language> Copy for RedLeaf<L> {}

impl<L: Language> PartialEq for RedLeaf<L> {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.span == other.span
    }
}

impl<L: Language> Eq for RedLeaf<L> {}

impl<L: Language> fmt::Debug for RedLeaf<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RedLeaf").field("kind", &self.kind).field("span", &self.span).finish()
    }
}

impl<'a, L: Language> RedNode<'a, L> {
    /// Creates a new red node from a green node and offset.
    #[inline]
    pub fn new(green: &'a GreenNode<'a, L>, offset: usize) -> Self {
        Self { green, offset }
    }

    /// Returns the absolute byte span of this red node.
    #[inline]
    pub fn span(&self) -> Range<usize> {
        Range { start: self.offset, end: self.offset + self.green.text_len() as usize }
    }

    /// Gets the child element at the specified index.
    pub fn child_at(&self, idx: usize) -> RedTree<'a, L> {
        let children = self.green.children();
        let green_child = &children[idx];

        // Calculate offset by summing lengths of previous siblings
        let mut offset = self.offset;
        for i in 0..idx {
            offset += children[i].len() as usize;
        }

        match green_child {
            GreenTree::Node(n) => RedTree::Node(RedNode::new(n, offset)),
            GreenTree::Leaf(t) => RedTree::Leaf(RedLeaf { kind: t.kind, span: Range { start: offset, end: offset + t.length as usize } }),
        }
    }

    /// Returns an iterator over the child elements of this red node.
    pub fn children(&self) -> RedChildren<'a, L> {
        RedChildren { node: *self, index: 0, offset: self.offset }
    }

    /// Finds the child element at the specified absolute byte offset.
    pub fn child_index_at_offset(&self, offset: usize) -> Option<usize> {
        if offset < self.offset || offset >= self.offset + self.green.text_len() as usize {
            return None;
        }

        let relative_offset = (offset - self.offset) as u32;
        let mut current_pos = 0;

        for (idx, child) in self.green.children().iter().enumerate() {
            let len = child.len();
            if relative_offset < current_pos + len {
                return Some(idx);
            }
            current_pos += len;
        }

        None
    }

    /// Finds the child element at the specified absolute byte offset.
    pub fn child_at_offset(&self, offset: usize) -> Option<RedTree<'a, L>> {
        self.child_index_at_offset(offset).map(|idx| self.child_at(idx))
    }
}

/// An iterator over the child elements of a red node.
pub struct RedChildren<'a, L: Language> {
    node: RedNode<'a, L>,
    index: usize,
    offset: usize,
}

impl<'a, L: Language> Iterator for RedChildren<'a, L> {
    type Item = RedTree<'a, L>;

    fn next(&mut self) -> Option<Self::Item> {
        let children = self.node.green.children();
        if self.index >= children.len() {
            return None;
        }

        let ch = &children[self.index];
        let offset = self.offset;
        let elem = match ch {
            GreenTree::Node(n) => RedTree::Node(RedNode::new(n, offset)),
            GreenTree::Leaf(t) => RedTree::Leaf(RedLeaf { kind: t.kind, span: Range { start: offset, end: offset + t.length as usize } }),
        };

        self.offset += ch.len() as usize;
        self.index += 1;
        Some(elem)
    }
}
