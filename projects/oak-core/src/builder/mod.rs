use crate::{
    Language,
    errors::OakDiagnostics,
    source::{Source, TextEdit},
    tree::GreenNode,
};

/// Output of a builder operation.
pub type BuildOutput<L: Language> = OakDiagnostics<L::TypedRoot>;

/// Trait for building higher-level structures (like ASTs) from a green tree.
///
/// While the [`Parser`] produces a generic [`GreenNode`] tree, the `Builder` is
/// responsible for "lowering" or "binding" this tree into a more ergonomic,
/// language-specific structure (e.g., an AST with typed nodes).
///
/// # Usage Scenario
///
/// The `Builder` is typically used after parsing to:
/// 1. Traverse the produced [`GreenNode`] tree.
/// 2. Construct typed AST nodes.
/// 3. Cache these typed nodes for incremental updates.
pub trait Builder<L: Language> {
    /// Builds the higher-level structure (typically an AST) from the source text.
    ///
    /// This method is the primary entry point for converting a raw source or a
    /// syntax tree into a typed AST. It handles incremental rebuilding if
    /// edits and a cache are provided.
    ///
    /// # Arguments
    ///
    /// * `text` - The source text to build from.
    /// * `edits` - A slice of [`TextEdit`]s representing changes since the last build.
    ///             Empty if building from scratch.
    /// * `cache` - A [`BuilderCache`] for incremental reuse and resource management.
    ///
    /// # Returns
    ///
    /// A [`BuildOutput`] containing the typed root node and any diagnostics.
    fn build<'a, S: Source + ?Sized>(&self, text: &S, edits: &[TextEdit], cache: &'a mut impl BuilderCache<L>) -> BuildOutput<L>;
}

/// Cache trait for builder operations used by lexers and parsers.
///
/// This trait defines the interface for building green tree nodes incrementally.
/// It provides methods for adding tokens and nodes to the tree structure.
pub trait BuilderCache<L: Language>: crate::parser::ParseCache<L> {
    /// Gets a cached typed node for a given green node.
    fn get_typed_node<T: 'static + Send + Sync + Clone>(&self, green: &GreenNode<L>) -> Option<T>;

    /// Caches a typed node for a given green node.
    fn set_typed_node<T: 'static + Send + Sync>(&mut self, green: &GreenNode<L>, typed: T);
}

impl<'a, L: Language, C: BuilderCache<L> + ?Sized> BuilderCache<L> for &'a mut C {
    fn get_typed_node<T: 'static + Send + Sync + Clone>(&self, green: &GreenNode<L>) -> Option<T> {
        (**self).get_typed_node(green)
    }

    fn set_typed_node<T: 'static + Send + Sync>(&mut self, green: &GreenNode<L>, typed: T) {
        (**self).set_typed_node(green, typed)
    }
}

impl<L: Language + Send + Sync> BuilderCache<L> for crate::parser::ParseSession<L> {
    fn get_typed_node<T: 'static + Send + Sync + Clone>(&self, green: &GreenNode<L>) -> Option<T> {
        self.get_typed_node(green)
    }

    fn set_typed_node<T: 'static + Send + Sync>(&mut self, green: &GreenNode<L>, typed: T) {
        self.set_typed_node(green, typed)
    }
}
