use crate::language::JinjaLanguage;
/// Jinja AST module
///
/// This module defines the abstract syntax tree (AST) for Jinja templates.
use oak_core::tree::GreenTree;

/// The root node of a Jinja template AST
#[derive(Debug, Clone, PartialEq)]
pub struct JinjaRoot<'a> {
    /// The green tree
    pub green_tree: GreenTree<'a, JinjaLanguage>,
}

impl<'a> JinjaRoot<'a> {
    /// Create a new Jinja root with the given green tree
    pub fn new(green_tree: GreenTree<'a, JinjaLanguage>) -> Self {
        Self { green_tree }
    }
}
