//! Tree traversal and transformation utilities.
//!
//! This module provides traits and utilities for visiting and transforming red-green trees.

use crate::{
    Language,
    tree::red_tree::{RedLeaf, RedNode, RedTree},
};

/// A visitor for traversing a red-green tree.
pub trait Visitor<'a, L: Language> {
    /// Visits a red node.
    fn visit_node(&mut self, node: RedNode<'a, L>);

    /// Visits a red leaf kind.
    fn visit_token(&mut self, token: RedLeaf<L>);

    /// Helper to walk children of a node.
    fn walk_node(&mut self, node: RedNode<'a, L>) {
        for child in node.children() {
            match child {
                RedTree::Node(n) => self.visit_node(n),
                RedTree::Leaf(t) => self.visit_token(t),
            }
        }
    }
}

/// A pre-order traversal iterator for red trees.
pub struct PreOrder<'a, L: Language> {
    stack: Vec<RedTree<'a, L>>,
}

impl<'a, L: Language> PreOrder<'a, L> {
    /// Creates a new pre-order iterator starting from the given root.
    pub fn new(root: RedTree<'a, L>) -> Self {
        Self { stack: vec![root] }
    }
}

impl<'a, L: Language> Iterator for PreOrder<'a, L> {
    type Item = RedTree<'a, L>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.stack.pop()?;

        if let RedTree::Node(node) = next {
            // Push children in reverse order so they are popped in correct order
            let children = node.green.children();
            let mut offset = node.offset + node.green.text_len() as usize;

            for child in children.iter().rev() {
                offset -= child.len() as usize;
                match child {
                    crate::GreenTree::Node(n) => {
                        self.stack.push(RedTree::Node(RedNode::new(n, offset)));
                    }
                    crate::GreenTree::Leaf(t) => {
                        self.stack.push(RedTree::Leaf(RedLeaf { kind: t.kind, span: core::range::Range { start: offset, end: offset + t.length as usize } }));
                    }
                }
            }
        }

        Some(next)
    }
}
