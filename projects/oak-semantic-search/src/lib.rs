#![feature(new_range_api)]
use oak_core::{
    errors::OakError,
    language::{ElementRole, ElementType, Language, UniversalElementRole},
    tree::{RedNode, red_tree::RedLeaf},
    visitor::Visitor,
};
use serde::{Deserialize, Serialize};

pub trait SemanticSearch: Send + Sync {
    fn search(&self, query: &str, limit: usize) -> impl std::future::Future<Output = Result<Vec<String>, OakError>> + Send;
}

/// A default implementation of SemanticSearch that does nothing.
pub struct NoSemanticSearch;

impl SemanticSearch for NoSemanticSearch {
    fn search(&self, _query: &str, _limit: usize) -> impl std::future::Future<Output = Result<Vec<String>, OakError>> + Send {
        async { Err(OakError::semantic_error("Semantic search is not enabled on this server")) }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeChunk {
    pub text: String,
    pub range_start: usize,
    pub range_end: usize,
    pub role: String,
}

pub struct SemanticSearcher {}

struct ChunkCollector<'a, L: Language> {
    source: &'a str,
    chunks: Vec<CodeChunk>,
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
        self.walk_node(node);
    }

    fn visit_token(&mut self, _token: RedLeaf<L>) {}
}

impl SemanticSearcher {
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
