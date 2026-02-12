#![doc = include_str!("readme.md")]
use crate::OrgModeLanguage;
use oak_core::tree::RedNode;

/// Org-mode AST root node.
pub struct OrgModeRoot<'a> {
    /// Corresponding red node.
    pub node: RedNode<'a, OrgModeLanguage>,
}

impl<'a> OrgModeRoot<'a> {
    /// Creates a new `OrgModeRoot`.
    pub fn new(node: RedNode<'a, OrgModeLanguage>) -> Self {
        Self { node }
    }
}
