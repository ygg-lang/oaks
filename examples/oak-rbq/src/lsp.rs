use crate::{kind::RbqSyntaxKind, language::RbqLanguage, semantic_tokens::RbqSemanticTokensProvider};
use core::range::Range;
use futures::Future;
use oak_core::{Parser, source::Source, tree::RedNode};
use oak_folding::{FoldingProvider, FoldingRange, FoldingRangeKind};
use oak_hover::{Hover, HoverProvider};
use oak_lsp::service::LanguageService;
use oak_vfs::Vfs;

pub struct RbqHoverProvider;

impl HoverProvider<RbqLanguage> for RbqHoverProvider {
    fn hover(&self, node: &RedNode<RbqLanguage>, _range: Range<usize>) -> Option<Hover> {
        let _kind = node.green.kind;
        None
    }
}

pub struct RbqFoldingProvider;

impl FoldingProvider<RbqLanguage> for RbqFoldingProvider {
    fn folding_ranges(&self, root: &RedNode<RbqLanguage>) -> Vec<FoldingRange> {
        let mut ranges = Vec::new();
        self.collect_folding_ranges(root, &mut ranges);
        ranges
    }
}

impl RbqFoldingProvider {
    fn collect_folding_ranges(&self, node: &RedNode<RbqLanguage>, ranges: &mut Vec<FoldingRange>) {
        match node.green.kind {
            RbqSyntaxKind::NamespaceDefinition | RbqSyntaxKind::StructDefinition | RbqSyntaxKind::EnumDefinition => {
                ranges.push(FoldingRange { range: node.span(), kind: None });
            }
            RbqSyntaxKind::Comment | RbqSyntaxKind::BlockComment => {
                ranges.push(FoldingRange { range: node.span(), kind: Some(FoldingRangeKind::Comment) });
            }
            _ => {}
        }

        for child in node.children() {
            if let oak_core::tree::RedTree::Node(n) = child {
                self.collect_folding_ranges(&n, ranges);
            }
        }
    }
}

use dashmap::DashMap;
use oak_core::parser::ParseSession;

pub struct RbqLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: RbqHoverProvider,
    folding_provider: RbqFoldingProvider,
    semantic_tokens_provider: RbqSemanticTokensProvider,
    caches: DashMap<String, ParseSession<RbqLanguage>>,
}

impl<V: Vfs> RbqLanguageService<V> {
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: RbqHoverProvider, folding_provider: RbqFoldingProvider, semantic_tokens_provider: RbqSemanticTokensProvider, caches: DashMap::new() }
    }
}

impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for RbqLanguageService<V> {
    type Lang = RbqLanguage;
    type Vfs = V;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }
    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }

    fn get_root(&self, _uri: &str) -> impl Future<Output = Option<RedNode<'_, RbqLanguage>>> + Send + '_ {
        async { None }
    }

    fn hover(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Option<oak_lsp::Hover>> + Send + '_ {
        let uri = uri.to_string();
        async move {
            let source = self.vfs().get_source(&uri)?;
            let parser = crate::parser::RbqParser::new(&crate::language::RbqLanguage);
            let mut cache = oak_core::parser::ParseSession::default();
            let output = parser.parse(&source, &[], &mut cache);
            let root_green = output.result.ok()?;
            let root = RedNode { green: root_green, offset: 0 };

            self.hover_provider.hover(&root, range).map(|h| oak_lsp::Hover { contents: h.contents, range: h.range })
        }
    }

    fn folding_ranges(&self, uri: &str) -> impl Future<Output = Vec<oak_lsp::FoldingRange>> + Send + '_ {
        let uri = uri.to_string();
        async move {
            let source = self.vfs().get_source(&uri).unwrap(); // Handle error properly in real impl
            let parser = crate::parser::RbqParser::new(&crate::language::RbqLanguage);
            let mut cache = oak_core::parser::ParseSession::default();
            let output = parser.parse(&source, &[], &mut cache);
            let root_green = match output.result {
                Ok(g) => g,
                Err(_) => return vec![],
            };
            let root = RedNode { green: root_green, offset: 0 };

            self.folding_provider.folding_ranges(&root).into_iter().map(|f| oak_lsp::FoldingRange { range: f.range, kind: f.kind }).collect()
        }
    }

    fn semantic_tokens<'a>(&'a self, uri: &'a str) -> impl Future<Output = Option<oak_lsp::SemanticTokens>> + Send + 'a {
        let uri = uri.to_string();
        async move {
            let source = self.vfs().get_source(&uri)?;
            let line_map = self.vfs().line_map(&uri)?;
            let parser = crate::parser::RbqParser::new(&crate::language::RbqLanguage);
            let mut cache = oak_core::parser::ParseSession::default();
            let output = parser.parse(&source, &[], &mut cache);
            let root_green = output.result.ok()?;
            let root = RedNode { green: root_green, offset: 0 };

            use oak_semantic_tokens::SemanticTokensProvider;
            let tokens = self.semantic_tokens_provider.semantic_tokens(&root, &source, &line_map);

            Some(oak_lsp::SemanticTokens {
                result_id: None,
                data: tokens.into_iter().map(|t| oak_lsp::types::SemanticToken { delta_line: t.delta_line, delta_start: t.delta_start, length: t.length, token_type: t.token_type, token_modifiers_bitset: t.token_modifiers_bitset }).collect(),
            })
        }
    }

    fn semantic_tokens_range<'a>(&'a self, _uri: &'a str, _range: Range<usize>) -> impl Future<Output = Option<oak_lsp::SemanticTokens>> + Send + 'a {
        async { None }
    }
}
