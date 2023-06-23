#![doc = include_str!("readme.md")]
#[cfg(feature = "oak-highlight")]
pub mod highlighter;

use crate::{HandlebarsElementType, HandlebarsLanguage};
use core::range::Range;
use oak_core::tree::RedNode;
#[cfg(feature = "lsp")]
use {
    futures::Future,
    oak_hover::{Hover, HoverProvider},
    oak_lsp::service::LanguageService,
    oak_vfs::Vfs,
};
/// Hover provider implementation for Handlebars.
#[cfg(feature = "lsp")]
pub struct HandlebarsHoverProvider;
#[cfg(feature = "lsp")]
impl HoverProvider<HandlebarsLanguage> for HandlebarsHoverProvider {
    fn hover(&self, node: &RedNode<HandlebarsLanguage>, _range: Range<usize>) -> Option<Hover> {
        let kind = node.green.kind;
        // Provide context-aware hover information
        let contents = match kind {
            HandlebarsElementType::Mustache => "### Handlebars Mustache\nA basic template expression.",
            HandlebarsElementType::Block => "### Handlebars Block\nA block helper expression.",
            HandlebarsElementType::Partial => "### Handlebars Partial\nIncludes another template.",
            HandlebarsElementType::CommentNode => "### Handlebars Comment\nA template comment.",
            _ => return None,
        };
        Some(Hover { contents: contents.to_string(), range: Some(node.span()) })
    }
}
/// Language service implementation for Handlebars.
#[cfg(feature = "lsp")]
pub struct HandlebarsLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: HandlebarsHoverProvider,
}
impl<V: Vfs> HandlebarsLanguageService<V> {
    /// Creates a new `HandlebarsLanguageService`.
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: HandlebarsHoverProvider }
    }
}
impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for HandlebarsLanguageService<V> {
    type Lang = HandlebarsLanguage;
    type Vfs = V;
    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }
    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }
    fn get_root(&self, _uri: &str) -> impl Future<Output = Option<RedNode<'_, HandlebarsLanguage>>> + Send + '_ {
        async move {
            // TODO: Implement proper caching and conversion to RedNode
            None
        }
    }
    fn hover(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Option<oak_lsp::Hover>> + Send + '_ {
        let uri = uri.to_string();
        async move { self.with_root(&uri, |root| self.hover_provider.hover(&root, range).map(|h| oak_lsp::Hover { contents: h.contents, range: h.range })).await.flatten() }
    }
}
