#![doc = include_str!("readme.md")]
#[cfg(feature = "oak-highlight")]
pub mod highlighter;

#[cfg(feature = "lsp")]
use {
    futures::Future,
    oak_hover::{Hover, HoverProvider},
    oak_lsp::service::LanguageService,
    oak_vfs::Vfs,
};
#[cfg(feature = "oak-pretty-print")]
pub mod formatter;
use crate::{JLanguage, parser::JElementType};
use core::range::Range;
use oak_core::tree::RedNode;
/// Hover provider implementation for J.
#[cfg(feature = "lsp")]
pub struct JHoverProvider;
#[cfg(feature = "lsp")]
impl HoverProvider<JLanguage> for JHoverProvider {
    fn hover(&self, node: &RedNode<JLanguage>, _range: Range<usize>) -> Option<Hover> {
        let kind = node.green.kind;
        // Provide context-aware hover information
        let contents = match kind {
            JElementType::Sentence => "### J Sentence\nA basic unit of execution in J.",
            JElementType::Assignment => "### J Assignment\nAssigns a value to a name locally (=.) or globally (=:).",
            JElementType::Verb => "### J Verb\nA function that operates on nouns.",
            _ => return None,
        };
        Some(Hover { contents: contents.to_string(), range: Some(node.span()) })
    }
}
/// Language service implementation for J.
#[cfg(feature = "lsp")]
pub struct JLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: JHoverProvider,
}
#[cfg(feature = "lsp")]
impl<V: Vfs> JLanguageService<V> {
    /// Creates a new `JLanguageService`.
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: JHoverProvider }
    }
}
#[cfg(feature = "lsp")]
impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for JLanguageService<V> {
    type Lang = JLanguage;
    type Vfs = V;
    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }
    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }
    fn get_root(&self, _uri: &str) -> impl Future<Output = Option<RedNode<'_, JLanguage>>> + Send + '_ {
        async move {
            // TODO: Implement proper caching of parsed trees in LanguageService
            None
        }
    }
    fn hover(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Option<oak_lsp::Hover>> + Send + '_ {
        let uri = uri.to_string();
        async move { self.with_root(&uri, |root| self.hover_provider.hover(&root, range).map(|h| oak_lsp::Hover { contents: h.contents, range: h.range })).await.flatten() }
    }
}
