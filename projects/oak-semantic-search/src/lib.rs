#![feature(new_range_api)]
#![allow(async_fn_in_trait)]
#![warn(missing_docs)]
//! Semantic search support for the Oak language framework.
//!
//! This crate provides traits and structures for performing semantic search
//! on source code, including code chunking and indexing.
use oak_core::{
    errors::OakError,
    language::{ElementRole, ElementType, Language, UniversalElementRole},
    tree::{RedNode, red_tree::RedLeaf},
    visitor::Visitor,
};
use serde::{Deserialize, Serialize};

/// Trait for semantic search implementations.
pub trait SemanticSearch: Send + Sync {
    /// Search for code segments that are semantically similar to the query.
    ///
    /// # Arguments
    /// * `query` - The search query string.
    /// * `limit` - The maximum number of results to return.
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<String>, OakError>;
}

/// A default implementation of SemanticSearch that does nothing.
pub struct NoSemanticSearch;

impl SemanticSearch for NoSemanticSearch {
    /// Always returns an error indicating semantic search is disabled.
    async fn search(&self, _query: &str, _limit: usize) -> Result<Vec<String>, OakError> {
        Err(OakError::semantic_error("Semantic search is not enabled on this server"))
    }
}

/// Represents a chunk of code extracted for semantic indexing.
#[derive(Debug, Serialize, Deserialize)]
pub struct CodeChunk {
    /// The text content of the chunk.
    pub text: String,
    /// The starting byte offset in the source file.
    pub range_start: usize,
    /// The ending byte offset in the source file.
    pub range_end: usize,
    /// The role of the code element (e.g., "Definition", "Statement").
    pub role: String,
}

/// A searcher that performs semantic search on code.
pub struct SemanticSearcher {}

/// A visitor that collects code chunks from the syntax tree.
struct ChunkCollector<'a, L: Language> {
    /// The source text.
    source: &'a str,
    /// The collected code chunks.
    chunks: Vec<CodeChunk>,
    /// Phantom data for the language type.
    _phantom: std::marker::PhantomData<L>,
}

impl<'a, L: Language> ChunkCollector<'a, L> {
    fn new(source: &'a str) -> Self {
        Self { source, chunks: Vec::new(), _phantom: std::marker::PhantomData }
    }
}

impl<'a, L: Language> Visitor<'a, L> for ChunkCollector<'a, L> {
    fn visit_node(&mut self, node: RedNode<'a, L>) {
        let role = node.green.kind.role().universal();

        // Chunking strategy: treat Definitions and Statements as potential chunks
        match role {
            UniversalElementRole::Definition | UniversalElementRole::Statement | UniversalElementRole::Documentation => {
                let range = node.span();
                let text = self.source[range.start..range.end].to_string();

                // Only index chunks that are meaningful in length
                if text.len() > 20 {
                    self.chunks.push(CodeChunk { text, range_start: range.start, range_end: range.end, role: format!("{:?}", role) });
                }
            }
            _ => {}
        }
        self.walk_node(node)
    }

    fn visit_token(&mut self, _token: RedLeaf<L>) {}
}

impl SemanticSearcher {
    /// Creates a new semantic searcher with the given database path.
    pub async fn new(_db_path: &str) -> Result<Self, OakError> {
        Ok(Self {})
    }

    /// Chunks code using oak-core's AST and indexes it
    pub async fn index_code<'a, L: Language>(&self, root: RedNode<'a, L>, source: &'a str, _table_name: &str) -> Result<(), OakError> {
        let mut collector = ChunkCollector::<L>::new(source);
        collector.visit_node(root);

        if collector.chunks.is_empty() {
            return Ok(());
        }

        // Concrete implementation (embedding and vector storage) has been removed.
        // This is now a placeholder that only performs AST-based chunking.

        Ok(())
    }
}

impl SemanticSearch for SemanticSearcher {
    async fn search(&self, _query: &str, _limit: usize) -> Result<Vec<String>, OakError> {
        // Concrete implementation (query embedding and vector search) has been removed.
        Ok(vec![])
    }
}
