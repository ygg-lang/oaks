//! Red-green tree implementation with position information for incremental parsing.
//!
//! This module provides the "red" side of the red-green tree architecture,
//! where red nodes contain absolute position information computed from
//! green nodes and their offsets.

use crate::tree::green_tree::{GreenNode, GreenTree};
use alloc::{rc::Rc, vec::Vec};
use core::range::Range;

/// A red tree element with absolute position information.
///
/// Red trees are the position-aware representation of kind trees,
/// computed by applying offsets to green trees. They are used for
/// incremental parsing, error reporting, and diagnostics.
#[derive(Debug, Clone)]
pub enum RedTree<K: Copy> {
    /// A red node with child elements
    Node(RedNode<K>),
    /// A red leaf kind
    Leaf(RedLeaf<K>),
}

impl<K: Copy> RedTree<K> {
    /// Returns the absolute byte span of this red tree element.
    ///
    /// # Returns
    ///
    /// A [`Range<usize>`] representing the absolute byte positions
    /// in the source text that this element occupies.
    #[inline]
    pub fn span(&self) -> Range<usize> {
        match self {
            RedTree::Node(n) => n.span(),
            RedTree::Leaf(t) => t.span,
        }
    }
}

/// A red node that wraps a green node with absolute offset information.
///
/// Red nodes represent kind tree nodes with computed absolute positions,
/// making them suitable for incremental parsing and position-based operations.
#[derive(Debug, Clone)]
pub struct RedNode<K: Copy> {
    /// The underlying green node that contains the structural information
    pub green: Rc<GreenNode<K>>,
    /// The absolute byte offset of this node in the source text
    pub offset: usize,
}

/// A red leaf kind with absolute position information.
///
/// Red leaves represent individual tokens (keywords, identifiers, literals, etc.)
/// with their absolute positions in the source text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedLeaf<K: Copy> {
    /// The kind kind/category (e.g., keyword, identifier, literal)
    pub kind: K,
    /// The absolute byte span of this kind in the source text
    pub span: Range<usize>,
}

impl<K: Copy> RedNode<K> {
    /// Creates a new red node from a green node and offset.
    ///
    /// # Arguments
    ///
    /// * `green` - The green node containing structural information
    /// * `offset` - The absolute byte offset in the source text
    ///
    /// # Returns
    ///
    /// A new [`RedNode`] with the given green node and offset
    #[inline]
    pub fn new(green: Rc<GreenNode<K>>, offset: usize) -> Self {
        Self { green, offset }
    }

    /// Returns the absolute byte span of this red node.
    ///
    /// The span is computed from the node's offset and the length
    /// of the underlying green node.
    ///
    /// # Returns
    ///
    /// A [`Range<usize>`] representing the absolute byte positions
    #[inline]
    pub fn span(&self) -> Range<usize> {
        Range { start: self.offset, end: self.offset + self.green.length }
    }

    /// Gets the child element at the specified index.
    ///
    /// # Arguments
    ///
    /// * `idx` - The index of the child element to retrieve
    ///
    /// # Returns
    ///
    /// An [`Option<RedTree<K>>`] containing the child element if it exists,
    /// or `None` if the index is out of bounds
    pub fn child(&self, idx: usize) -> Option<RedTree<K>> {
        let mut cur = self.offset;
        let ch = self.green.children.get(idx)?;
        // Calculate the starting offset of this child element
        for c in &self.green.children[..idx] {
            cur += c.len();
        }
        Some(match ch {
            GreenTree::Node(n) => RedTree::Node(RedNode::new(Rc::clone(n), cur)),
            GreenTree::Leaf(t) => RedTree::Leaf(RedLeaf { kind: t.kind, span: Range { start: cur, end: cur + t.length } }),
        })
    }

    /// Returns an iterator over all child elements.
    ///
    /// # Returns
    ///
    /// A [`RedChildren<K>`] iterator that yields all child elements
    /// in order
    pub fn children(&self) -> RedChildren<'_, K> {
        RedChildren::new(self)
    }
}

// Additional methods for incremental parsing support
impl<K: Copy> RedNode<K> {
    /// Finds the index of the child element that contains the given absolute offset.
    ///
    /// This method is essential for incremental parsing, allowing efficient
    /// location of affected regions when source text changes.
    ///
    /// # Arguments
    ///
    /// * `offset` - The absolute byte offset to search for
    ///
    /// # Returns
    ///
    /// An [`Option<usize>`] containing the child index if found, or `None`
    /// if the offset is outside this node's span
    pub fn child_index_at_offset(&self, offset: usize) -> Option<usize> {
        let start = self.offset;
        let end = self.offset + self.green.length;
        if offset < start || offset >= end {
            return None;
        }
        let mut cur = start;
        for (i, ch) in self.green.children.iter().enumerate() {
            let next = cur + ch.len();
            if offset < next {
                return Some(i);
            }
            cur = next;
        }
        None
    }

    /// Gets the absolute starting offset of the child element at the given index.
    ///
    /// # Arguments
    ///
    /// * `idx` - The index of the child element
    ///
    /// # Returns
    ///
    /// An [`Option<usize>`] containing the absolute offset if the index is valid
    pub fn offset_of_child(&self, idx: usize) -> Option<usize> {
        if idx >= self.green.children.len() {
            return None;
        }
        let mut cur = self.offset;
        for c in &self.green.children[..idx] {
            cur += c.len();
        }
        Some(cur)
    }

    /// Collects indices of child elements that overlap with the given span.
    ///
    /// This method is crucial for incremental parsing, identifying which
    /// child elements are affected by a text change.
    ///
    /// # Arguments
    ///
    /// * `span` - The byte range to check for overlaps
    ///
    /// # Returns
    ///
    /// A [`Vec<usize>`] containing indices of all overlapping child elements
    pub fn overlapping_indices(&self, span: Range<usize>) -> Vec<usize> {
        let mut out = Vec::new();
        let node_start = self.offset;
        let node_end = self.offset + self.green.length;
        // Return empty if there's no intersection
        if span.end <= node_start || span.start >= node_end {
            return out;
        }
        let mut cur = node_start;
        for (i, ch) in self.green.children.iter().enumerate() {
            let ch_start = cur;
            let ch_end = cur + ch.len();
            // Check if ranges overlap
            if !(ch_end <= span.start || ch_start >= span.end) {
                out.push(i);
            }
            cur = ch_end;
        }
        out
    }
}

/// Iterator over the child elements of a red node.
///
/// This iterator yields [`RedTree<K>`] elements in order, providing
/// access to all children with their absolute position information.
#[derive(Debug)]
pub struct RedChildren<'a, K: Copy> {
    node: &'a RedNode<K>,
    index: usize,
    offset: usize,
}

impl<'a, K: Copy> RedChildren<'a, K> {
    fn new(node: &'a RedNode<K>) -> Self {
        Self { node, index: 0, offset: node.offset }
    }
}

impl<'a, K: Copy> Iterator for RedChildren<'a, K> {
    type Item = RedTree<K>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.node.green.children.len() {
            return None;
        }
        let ch = &self.node.green.children[self.index];
        let elem = match ch {
            GreenTree::Node(n) => RedTree::Node(RedNode::new(Rc::clone(n), self.offset)),
            GreenTree::Leaf(t) => {
                RedTree::Leaf(RedLeaf { kind: t.kind, span: Range { start: self.offset, end: self.offset + t.length } })
            }
        };
        self.offset += ch.len();
        self.index += 1;
        Some(elem)
    }
}
