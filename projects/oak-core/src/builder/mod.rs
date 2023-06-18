use crate::{GreenBuilder, IncrementalCache, Language, OakDiagnostics, source::Source};

/// Builder trait for constructing typed kind trees from source text.
///
/// This trait provides a unified interface for building complete kind trees
/// from source text, supporting both full parsing and incremental updates.
/// It coordinates the lexing, parsing, and AST construction phases.
pub trait Builder<L: Language> {
    /// Builds a complete kind tree from the given source text.
    ///
    /// This method performs a full parse of the source text, creating a new
    /// kind tree from scratch. It uses a default cache configuration.
    ///
    /// # Arguments
    ///
    /// * `text` - The source text to parse
    ///
    /// # Returns
    ///
    /// A diagnostics result containing either the parsed kind tree or errors
    fn build(&self, text: impl Source) -> OakDiagnostics<L::TypedRoot> {
        let mut pool = GreenBuilder::new(0);
        let cache = IncrementalCache::new(&mut pool);
        self.build_incremental(text, 0, cache)
    }

    /// Builds a kind tree incrementally using an existing cache.
    ///
    /// This method enables efficient re-parsing by reusing information from
    /// previous parsing operations, only processing the changed portions
    /// of the source text.
    ///
    /// # Arguments
    ///
    /// * `text` - The source text to parse
    /// * `changed` - The number of bytes that have changed since the last parse
    /// * `cache` - The incremental cache containing previous parsing results
    ///
    /// # Returns
    ///
    /// A diagnostics result containing either the parsed kind tree or errors
    fn build_incremental(&self, text: impl Source, changed: usize, cache: IncrementalCache<L>) -> OakDiagnostics<L::TypedRoot>;
}
