use crate::HtmlLanguage;
use core::range::Range;
use futures::Future;
use oak_core::tree::RedNode;
use oak_hover::{Hover, HoverProvider};
use oak_lsp::service::LanguageService;
use oak_vfs::Vfs;

/// Hover provider implementation for HTML.
pub struct HtmlHoverProvider;

impl HoverProvider<HtmlLanguage> for HtmlHoverProvider {
    fn hover(&self, _node: &RedNode<HtmlLanguage>, _range: Range<usize>) -> Option<Hover> {
        None
    }
}

/// Language service implementation for HTML.
pub struct HtmlLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    _hover_provider: HtmlHoverProvider,
}

impl<V: Vfs> HtmlLanguageService<V> {
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), _hover_provider: HtmlHoverProvider }
    }
}

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
}
