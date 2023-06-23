#![doc = include_str!("readme.md")]
#[cfg(feature = "oak-highlight")]
pub mod highlighter;

use crate::{NginxLanguage, parser::element_type::NginxElementType};
use oak_core::{Range, tree::RedNode};
#[cfg(feature = "lsp")]
use {
    oak_hover::HoverProvider,
    oak_lsp::{service::LanguageService, types::Hover},
    oak_vfs::Vfs,
    std::future::Future,
};
/// Hover provider implementation for Nginx.
#[cfg(feature = "lsp")]
pub struct NginxHoverProvider;
#[cfg(feature = "lsp")]
impl HoverProvider<NginxLanguage> for NginxHoverProvider {
    fn hover(&self, root: &RedNode<'_, NginxLanguage>, _range: Range<usize>) -> Option<oak_hover::Hover> {
        let kind = root.green.kind;
        let contents = match kind {
            NginxElementType::Directive => "### Nginx Directive\nA configuration instruction for the Nginx server.",
            NginxElementType::Block => "### Nginx Block\nA group of directives enclosed in braces.",
            NginxElementType::Root => "### Nginx Configuration\nThe root of an Nginx configuration file.",
            _ => return None,
        };
        Some(oak_hover::Hover { contents: contents.to_string(), range: Some(root.span()) })
    }
}
/// Language service implementation for Nginx.
#[cfg(feature = "lsp")]
pub struct NginxLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: NginxHoverProvider,
}
impl<V: Vfs> NginxLanguageService<V> {
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: NginxHoverProvider }
    }
}
impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for NginxLanguageService<V> {
    type Lang = NginxLanguage;
    type Vfs = V;
    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }
    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }
    fn get_root(&self, _uri: &str) -> impl Future<Output = Option<RedNode<'_, NginxLanguage>>> + Send + '_ {
        async move { None }
    }
    fn hover(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Option<Hover>> + Send + '_ {
        let uri = uri.to_string();
        async move {
            let h = self.with_root(&uri, |root| self.hover_provider.hover(&root, range)).await.flatten()?;
            Some(Hover { contents: h.contents, range: h.range })
        }
    }
}
