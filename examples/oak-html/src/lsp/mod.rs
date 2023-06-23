#![doc = include_str!("readme.md")]
use crate::HtmlLanguage;
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
/// Hover provider implementation for HTML.
///
/// This provider is responsible for generating hover information for HTML elements,
/// such as tag descriptions or attribute documentation.
#[cfg(feature = "lsp")]
pub struct HtmlHoverProvider;
#[cfg(feature = "lsp")]
impl HoverProvider<HtmlLanguage> for HtmlHoverProvider {
    /// Provides hover information for a specific node in the HTML tree.
    fn hover(&self, _node: &RedNode<HtmlLanguage>, _range: Range<usize>) -> Option<Hover> {
        None
    }
}
/// Language service implementation for HTML.
///
/// This service provides LSP features like hover, completion, and diagnostics
/// for the HTML language, utilizing the VFS for file management.
#[cfg(feature = "lsp")]
pub struct HtmlLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    _hover_provider: HtmlHoverProvider,
}
#[cfg(feature = "lsp")]
impl<V: Vfs> HtmlLanguageService<V> {
    /// Creates a new `HtmlLanguageService` with the given VFS.
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), _hover_provider: HtmlHoverProvider }
    }
}
#[cfg(feature = "lsp")]
impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for HtmlLanguageService<V> {
    type Lang = HtmlLanguage;
    type Vfs = V;
    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }
    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }
    fn get_root(&self, _uri: &str) -> impl Future<Output = Option<RedNode<'_, HtmlLanguage>>> + Send + '_ {
        async move { None }
    }
    fn hover(&self, _uri: &str, _range: Range<usize>) -> impl Future<Output = Option<oak_lsp::Hover>> + Send + '_ {
        async move { None }
    }
    fn completion(&self, _uri: &str, _offset: usize) -> impl Future<Output = Vec<oak_lsp::types::CompletionItem>> + Send + '_ {
        async move { vec![] }
    }
}
