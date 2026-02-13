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
///
/// Red trees are the "red" side of the red-green tree architecture. They are
/// lazily computed from green trees and include absolute byte offsets in
/// the source code.
pub enum RedTree<'a, L: Language> {
    /// A red node with child elements.
    Node(RedNode<'a, L>),
    /// A red token.
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
    ///
    /// The span includes the start and end offsets in the source text.
    #[inline]
    pub fn span(&self) -> Range<usize> {
        match self {
            RedTree::Node(n) => n.span(),
            RedTree::Leaf(t) => t.span,
        }
    }

    /// Returns the kind of this red tree element.
    ///
    /// # Type Parameters
    ///
    /// * `T` - A type that can be converted from both node and token kinds.
    pub fn kind<T>(&self) -> T
    where
        T: From<L::ElementType> + From<L::TokenType>,
    {
        match self {
            RedTree::Node(n) => T::from(n.green.kind),
            RedTree::Leaf(l) => T::from(l.kind),
        }
    }

    /// Returns the text content of this red tree element from the source.
    ///
    /// # Arguments
    ///
    /// * `source` - The source text provider.
    pub fn text<'s, S: crate::source::Source + ?Sized>(&self, source: &'s S) -> std::borrow::Cow<'s, str> {
        source.get_text_in(self.span())
    }

    /// Returns an iterator over the child elements if this is a node.
    ///
    /// Returns an empty iterator if this is a token.
    pub fn children(&self) -> RedChildren<'a, L> {
        match self {
            RedTree::Node(n) => n.children(),
            RedTree::Leaf(_) => RedChildren::empty(),
        }
    }

    /// Returns this element as a node if it is one.
    pub fn as_node(&self) -> Option<RedNode<'a, L>> {
        match self {
            RedTree::Node(n) => Some(*n),
            RedTree::Leaf(_) => None,
        }
    }

    /// Returns this element as a token if it is one.
    pub fn as_token(&self) -> Option<RedLeaf<L>> {
        match self {
            RedTree::Node(_) => None,
            RedTree::Leaf(l) => Some(*l),
        }
    }

    /// Alias for `as_token`.
    #[deprecated(note = "Use `as_token` instead")]
    pub fn as_leaf(&self) -> Option<RedLeaf<L>> {
        self.as_token()
    }
}

/// A red node that wraps a green node with absolute offset information.
///
/// Red nodes are position-aware views into the immutable green tree structure.
/// They are small, copyable handles that can be used for traversal and
/// analysis.
///
/// # Design Note: Reference vs Owned
/// We store `&'a GreenNode<'a, L>` here instead of `GreenNode<'a, L>` to keep
/// `RedNode` as small as possible (16 bytes: 8 for pointer + 8 for offset).
/// This makes it efficient to pass `RedNode` by value during tree traversal.
pub struct RedNode<'a, L: Language> {
    /// The underlying green node that contains the structural information.
    pub green: &'a GreenNode<'a, L>,
    /// The absolute byte offset of this node in the source text.
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
///
/// Red leaves represent individual tokens with their location in the source.
pub struct RedLeaf<L: Language> {
    /// The token kind/category.
    pub kind: L::TokenType,
    /// The absolute byte span of this token in the source text.
    pub span: Range<usize>,
}

// Manually implement Clone/Copy to avoid L: Copy bound
impl<L: Language> Clone for RedLeaf<L> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<L: Language> Copy for RedLeaf<L> {}

impl<L: Language> RedLeaf<L> {
    /// Returns the kind of this red leaf.
    #[inline]
    pub fn kind(&self) -> L::TokenType {
        self.kind
    }

    /// Returns the absolute byte span of this red leaf.
    #[inline]
    pub fn span(&self) -> Range<usize> {
        self.span.clone()
    }

    /// Returns the text content of this red leaf from the source.
    pub fn text<'s, S: crate::source::Source + ?Sized>(&self, source: &'s S) -> std::borrow::Cow<'s, str> {
        source.get_text_in(self.span())
    }
}

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

/// An iterator over the child elements of a red node.
///
/// This iterator lazily computes the absolute offsets of each child
/// as it traverses the tree.
pub struct RedChildren<'a, L: Language> {
    /// The parent red node being iterated.
    node: Option<RedNode<'a, L>>,
    /// The current index in the children slice.
    index: usize,
    /// The current absolute byte offset.
    offset: usize,
}

impl<'a, L: Language> RedChildren<'a, L> {
    /// Creates an empty iterator.
    pub fn empty() -> Self {
        Self { node: None, index: 0, offset: 0 }
    }
}

impl<'a, L: Language> RedNode<'a, L> {
    /// Returns the text content of this red node from the source.
    pub fn text<'s, S: crate::source::Source + ?Sized>(&self, source: &'s S) -> std::borrow::Cow<'s, str> {
        source.get_text_in(self.span())
    }

    /// Creates a new red node from a green node and absolute offset.
    #[inline]
    pub fn new(green: &'a GreenNode<'a, L>, offset: usize) -> Self {
        Self { green, offset }
    }

    /// Returns the absolute byte span of this red node.
    #[inline]
    pub fn span(&self) -> Range<usize> {
        Range { start: self.offset, end: self.offset + self.green.text_len() as usize }
    }

    /// Returns the element type of this red node.
    #[inline]
    pub fn element_type(&self) -> L::ElementType {
        self.green.kind
    }

    /// Returns the underlying green node.
    #[inline]
    pub fn green(&self) -> &'a GreenNode<'a, L> {
        self.green
    }

    /// Returns the kind of this red node.
    pub fn kind<T>(&self) -> T
    where
        T: From<L::ElementType>,
    {
        T::from(self.green.kind)
    }

    /// Gets the child element at the specified index.
    ///
    /// # Arguments
    ///
    /// * `idx` - The zero-based index of the child.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    pub fn child_at(&self, idx: usize) -> RedTree<'a, L> {
        let children = self.green.children();
        let green_child = &children[idx];

        // Calculate offset by summing lengths of previous siblings
        let mut offset = self.offset;
        for i in 0..idx {
            offset += children[i].len() as usize
        }

        match green_child {
            GreenTree::Node(n) => RedTree::Node(RedNode::new(n, offset)),
            GreenTree::Leaf(t) => RedTree::Leaf(RedLeaf { kind: t.kind, span: Range { start: offset, end: offset + t.length as usize } }),
        }
    }

    /// Returns an iterator over the child elements of this red node.
    pub fn children(&self) -> RedChildren<'a, L> {
        RedChildren { node: Some(*self), index: 0, offset: self.offset }
    }

    /// Finds the index of the child element containing the specified absolute byte offset.
    ///
    /// # Arguments
    ///
    /// * `offset` - The absolute byte offset to search for.
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
            current_pos += len
        }

        None
    }

    /// Finds the child element containing the specified absolute byte offset.
    ///
    /// # Arguments
    ///
    /// * `offset` - The absolute byte offset to search for.
    pub fn child_at_offset(&self, offset: usize) -> Option<RedTree<'a, L>> {
        self.child_index_at_offset(offset).map(|idx| self.child_at(idx))
    }

    /// Finds the leaf element at the specified absolute byte offset by traversing down the tree.
    ///
    /// This method performs a deep search, following child nodes until a leaf is found.
    ///
    /// # Arguments
    ///
    /// * `offset` - The absolute byte offset to search for.
    pub fn leaf_at_offset(&self, offset: usize) -> Option<RedLeaf<L>> {
        let mut current = *self;
        loop {
            match current.child_at_offset(offset)? {
                RedTree::Node(n) => current = n,
                RedTree::Leaf(l) => return Some(l),
            }
        }
    }
}

impl<'a, L: Language> Iterator for RedChildren<'a, L> {
    type Item = RedTree<'a, L>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node.as_ref()?;
        let children = node.green.children();
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
