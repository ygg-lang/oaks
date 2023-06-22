//! efficient Cursor for GreenNode traversal
//!
//! This module provides a cursor implementation for traversing GreenTrees
//! in an Arena-based memory model. It tracks absolute offsets and allows
//! efficient navigation (up, down, next sibling) without recursion.

use crate::{
    Language,
    tree::{GreenNode, GreenTree},
};

/// A cursor for traversing a GreenTree.
///
/// It maintains a path from the root to the current node, allowing for
/// `parent()` navigation and offset tracking.
#[derive(Debug, Clone)]
pub struct Cursor<'a, L: Language> {
    /// The current element (Node or Leaf).
    current: GreenTree<'a, L>,
    /// The absolute start offset of the current element.
    offset: usize,
    /// The stack of parent nodes and the index of the current node within them.
    /// Format: (Parent Node, Index in Parent, Parent Start Offset)
    stack: Vec<(&'a GreenNode<'a, L>, usize, usize)>,
}

impl<'a, L: Language> Cursor<'a, L> {
    /// Creates a new cursor starting at the given root node.
    pub fn new(root: &'a GreenNode<'a, L>) -> Self {
        Self { current: GreenTree::Node(root), offset: 0, stack: Vec::with_capacity(16) }
    }

    /// Returns the current element.
    #[inline]
    pub fn current(&self) -> GreenTree<'a, L> {
        self.current
    }

    /// Returns the current element as a node, if it is one.
    #[inline]
    pub fn as_node(&self) -> Option<&'a GreenNode<'a, L>> {
        match self.current {
            GreenTree::Node(n) => Some(n),
            GreenTree::Leaf(_) => None,
        }
    }

    /// Returns the absolute start offset of the current element.
    #[inline]
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Returns the text length of the current element.
    #[inline]
    pub fn len(&self) -> usize {
        self.current.len() as usize
    }

    /// Returns the absolute end offset of the current element.
    #[inline]
    pub fn end_offset(&self) -> usize {
        self.offset + self.len()
    }

    /// Tries to move to the first child of the current node.
    /// Returns true if successful (current was a node with children).
    pub fn step_into(&mut self) -> bool {
        match self.current {
            GreenTree::Node(node) => {
                if let Some(first_child) = node.children.first() {
                    self.stack.push((node, 0, self.offset));
                    self.current = *first_child;
                    // offset remains the same for the first child
                    true
                }
                else {
                    false
                }
            }
            GreenTree::Leaf(_) => false,
        }
    }

    /// Tries to move to the next sibling.
    /// Returns true if successful.
    pub fn step_over(&mut self) -> bool {
        if let Some((parent, idx, _parent_offset)) = self.stack.last() {
            let next_idx = idx + 1;
            if let Some(sibling) = parent.children.get(next_idx) {
                // Update offset: current offset + current len
                self.offset += self.current.len() as usize;

                // Update stack top
                let last = self.stack.last_mut().unwrap();
                last.1 = next_idx;

                self.current = *sibling;
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    /// Tries to move to the parent node.
    /// Returns true if successful (not at root).
    pub fn step_out(&mut self) -> bool {
        if let Some((parent, _, parent_offset)) = self.stack.pop() {
            self.current = GreenTree::Node(parent);
            self.offset = parent_offset;
            true
        }
        else {
            false
        }
    }

    /// Pre-order traversal step: try into, then over, then out + over.
    /// Returns true if moved to a new node, false if finished traversal.
    pub fn step(&mut self) -> bool {
        if self.step_into() {
            return true;
        }
        if self.step_over() {
            return true;
        }
        while self.step_out() {
            if self.step_over() {
                return true;
            }
        }
        false
    }

    /// Skips the current subtree (like step_over), but if that fails, goes up and over.
    /// Effectively "next node in post-order" or "next node not in this subtree".
    pub fn step_next(&mut self) -> bool {
        if self.step_over() {
            true
        }
        else {
            while self.step_out() {
                if self.step_over() {
                    return true;
                }
            }
            false
        }
    }
}
