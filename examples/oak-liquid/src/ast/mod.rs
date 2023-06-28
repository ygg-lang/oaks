use crate::language::LiquidLanguage;
/// Liquid AST module
///
/// This module defines the abstract syntax tree (AST) for Liquid templates.
use oak_core::tree::GreenTree;

/// The root node of a Liquid template AST
#[derive(Debug, Clone, PartialEq)]
pub struct LiquidRoot<'a> {
    /// The green tree
    pub green_tree: GreenTree<'a, LiquidLanguage>,
}

impl<'a> LiquidRoot<'a> {
    /// Create a new Liquid root with the given green tree
    pub fn new(green_tree: GreenTree<'a, LiquidLanguage>) -> Self {
        Self { green_tree }
    }
}
