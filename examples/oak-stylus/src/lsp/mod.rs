#![doc = include_str!("readme.md")]
use crate::language::StylusLanguage;
use core::range::Range;
use oak_core::tree::RedNode;
/// Highlighter module.
#[cfg(feature = "oak-highlight")]
pub mod highlighter;

#[cfg(feature = "lsp")]
use {
    futures::Future,
    oak_hover::{Hover, HoverProvider},
    oak_lsp::service::LanguageService,
    oak_vfs::Vfs,
};
/// MCP module.
/// Formatter module.
#[cfg(feature = "oak-pretty-print")]
pub mod formatter;
#[cfg(feature = "lsp")]
pub struct StylusHoverProvider;
#[cfg(feature = "lsp")]
impl HoverProvider<StylusLanguage> for StylusHoverProvider {
    fn hover(&self, _node: &RedNode<StylusLanguage>, _range: Range<usize>) -> Option<Hover> {
        None
    }
}
#[cfg(feature = "lsp")]
pub struct StylusLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: StylusHoverProvider,
}
impl<V: Vfs> StylusLanguageService<V> {
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: StylusHoverProvider }
    }
}
impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for StylusLanguageService<V> {
    type Lang = StylusLanguage;
    type Vfs = V;
    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }
    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }
    fn get_root(&self, uri: &str) -> impl Future<Output = Option<RedNode<'_, StylusLanguage>>> + Send + '_ {
        let source = self.vfs().get_source(uri);
        async move {
            let _source = source?;
            None
        }
    }
    fn hover(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Option<oak_lsp::Hover>> + Send + '_ {
        let uri = uri.to_string();
        async move { self.with_root(&uri, |root| self.hover_provider.hover(&root, range).map(|h| oak_lsp::Hover { contents: h.contents, range: h.range })).await.flatten() }
    }
}
