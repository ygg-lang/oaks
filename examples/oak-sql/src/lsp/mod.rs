#![doc = include_str!("readme.md")]
#[cfg(feature = "oak-highlight")]
pub mod highlighter;

#[cfg(feature = "lsp")]
use {
    crate::language::SqlLanguage,
    core::range::Range,
    futures::Future,
    oak_core::tree::RedNode,
    oak_hover::{Hover, HoverProvider},
    oak_lsp::service::LanguageService,
    oak_vfs::Vfs,
};

#[cfg(feature = "oak-pretty-print")]
pub mod formatter;

#[cfg(feature = "lsp")]
/// A hover provider for SQL.
pub struct SqlHoverProvider;

#[cfg(feature = "lsp")]
impl HoverProvider<SqlLanguage> for SqlHoverProvider {
    fn hover(&self, node: &RedNode<SqlLanguage>, _range: Range<usize>) -> Option<Hover> {
        let _kind = node.green.kind;
        None
    }
}

#[cfg(feature = "lsp")]
/// Language service implementation for SQL.
pub struct SqlLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: SqlHoverProvider,
}

#[cfg(feature = "lsp")]
impl<V: Vfs> SqlLanguageService<V> {
    /// Creates a new SQL language service.
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: SqlHoverProvider }
    }
}
#[cfg(feature = "lsp")]
impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for SqlLanguageService<V> {
    type Lang = SqlLanguage;
    type Vfs = V;
    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }
    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }
    fn get_root(&self, uri: &str) -> impl Future<Output = Option<RedNode<'_, SqlLanguage>>> + Send + '_ {
        let source = self.vfs().get_source(uri);
        async move {
            let source = source?;
            let language = SqlLanguage::default();
            let parser = crate::parser::SqlParser::new(&language);
            let lexer = crate::lexer::SqlLexer::new(&language);
            let mut cache = oak_core::parser::session::ParseSession::<SqlLanguage>::default();
            let _output = oak_core::parser::parse(&parser, &lexer, &source, &[], &mut cache);
            None
        }
    }
    fn hover(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Option<oak_lsp::Hover>> + Send + '_ {
        let uri = uri.to_string();
        async move { self.with_root(&uri, |root| self.hover_provider.hover(&root, range).map(|h| oak_lsp::Hover { contents: h.contents, range: h.range })).await.flatten() }
    }
}
