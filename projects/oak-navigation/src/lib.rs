#![feature(new_range_api)]
#![warn(missing_docs)]
//! Navigation support for the Oak language framework.
//!
//! This crate provides traits and structures for code navigation features
//! like "Go to Definition" and "Find All References".
use core::range::Range;
use oak_core::{Language, TokenType, language::UniversalTokenRole, tree::RedNode, visitor::Visitor};

/// Represents a location in a source file.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Location {
    /// The URI of the resource.
    #[serde(with = "oak_core::serde_arc_str")]
    pub uri: oak_core::Arc<str>,
    /// The byte range within the resource.
    #[serde(with = "oak_core::serde_range")]
    pub range: Range<usize>,
}

/// Trait for languages that support jumping to definition.
pub trait DefinitionProvider<L: Language> {
    /// Returns the definition(s) of the symbol at the given offset.
    fn definition(&self, root: &RedNode<L>, offset: usize) -> Vec<Location>;
}

/// Trait for languages that support finding references.
pub trait ReferencesProvider<L: Language> {
    /// Returns the references to the symbol at the given offset.
    fn references(&self, root: &RedNode<L>, offset: usize, include_declaration: bool) -> Vec<Location>;
}

/// A helper to find all references of a name in a tree.
pub struct SimpleReferenceFinder<'a, L: Language> {
    /// The name of the symbol to find references for.
    name: &'a str,
    /// The full source text.
    source: &'a str,
    /// The URI of the source file.
    uri: oak_core::Arc<str>,
    /// The list of locations where the name was found.
    references: Vec<Location>,
    /// Phantom data for the language type.
    _phantom: std::marker::PhantomData<L>,
}

impl<'a, L: Language> SimpleReferenceFinder<'a, L> {
    /// Finds all references of the given name in the tree.
    ///
    /// # Arguments
    /// * `root` - The root node of the syntax tree.
    /// * `name` - The name of the symbol to find.
    /// * `source` - The source text.
    /// * `uri` - The URI of the source file.
    pub fn find(root: &RedNode<'a, L>, name: &'a str, source: &'a str, uri: impl Into<oak_core::Arc<str>>) -> Vec<Location> {
        let mut finder = Self { name, source, uri: uri.into(), references: Vec::new(), _phantom: std::marker::PhantomData };
        finder.visit_node(*root);
        finder.references
    }
}

impl<'a, L: Language> Visitor<'a, L> for SimpleReferenceFinder<'a, L> {
    fn visit_node(&mut self, node: RedNode<'a, L>) {
        self.walk_node(node);
    }

    fn visit_token(&mut self, token: oak_core::tree::RedLeaf<L>) {
        if token.kind.is_universal(UniversalTokenRole::Name) {
            let text = &self.source[token.span.clone()];
            if text == self.name {
                self.references.push(Location { uri: self.uri.clone(), range: token.span });
            }
        }
    }
}
