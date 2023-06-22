use crate::SchemeLanguage;
use core::range::Range;
use futures::Future;
use oak_core::tree::RedNode;
use oak_hover::{Hover, HoverProvider};
use oak_lsp::service::LanguageService;
use oak_vfs::Vfs;

/// Hover provider implementation for Scheme.
pub struct SchemeHoverProvider;

impl HoverProvider<SchemeLanguage> for SchemeHoverProvider {
    fn hover(&self, node: &RedNode<SchemeLanguage>, _range: Range<usize>) -> Option<Hover> {
        let _kind = node.green.kind;
        // Basic Scheme hover implementation
        None
    }
}

/// Language service implementation for Scheme.
pub struct SchemeLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: SchemeHoverProvider,
}

impl<V: Vfs> SchemeLanguageService<V> {
    /// Creates a new `SchemeLanguageService`.
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: SchemeHoverProvider }
    }
}

impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for SchemeLanguageService<V> {
    type Lang = SchemeLanguage;
    type Vfs = V;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }

    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }

    fn get_root(&self, uri: &str) -> impl Future<Output = Option<RedNode<'_, SchemeLanguage>>> + Send + '_ {
        let _source = self.vfs().get_source(uri);
        async move {
            // Placeholder implementation
            None
        }
    }

    fn hover(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Option<oak_lsp::Hover>> + Send + '_ {
        let uri = uri.to_string();
        async move { self.with_root(&uri, |root| self.hover_provider.hover(&root, range).map(|h| oak_lsp::Hover { contents: h.contents, range: h.range })).await.flatten() }
    }
}
