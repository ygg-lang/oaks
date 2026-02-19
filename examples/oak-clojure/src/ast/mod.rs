#![doc = include_str!("readme.md")]
use crate::{language::ClojureLanguage, parser::ClojureElementType};
use oak_core::tree::{RedLeaf, RedNode};

/// Type alias for a Clojure syntax node.
pub type ClojureNode<'a> = RedNode<'a, ClojureLanguage>;

/// Type alias for a Clojure token leaf.
pub type ClojureToken = RedLeaf<ClojureLanguage>;

/// Root node of a Clojure AST.
#[derive(Debug, Clone, Copy)]
pub struct ClojureRoot<'a> {
    syntax: ClojureNode<'a>,
}

impl<'a> ClojureRoot<'a> {
    /// Attempts to cast a ClojureNode to a ClojureRoot.
    pub fn cast(node: ClojureNode<'a>) -> Option<Self> {
        if node.green.kind == ClojureElementType::SourceFile { Some(ClojureRoot { syntax: node }) } else { None }
    }

    /// Returns a reference to the underlying syntax node.
    pub fn syntax(&self) -> &ClojureNode<'a> {
        &self.syntax
    }
}
