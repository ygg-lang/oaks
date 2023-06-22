use crate::{ScssLanguage, kind::ScssSyntaxKind};
use core::range::Range;
use futures::Future;
use oak_core::tree::RedNode;
use oak_hover::{Hover, HoverProvider};
use oak_lsp::service::LanguageService;
use oak_vfs::Vfs;

/// Hover provider implementation for SCSS.
pub struct ScssHoverProvider;

impl HoverProvider<ScssLanguage> for ScssHoverProvider {
    fn hover(&self, node: &RedNode<'_, ScssLanguage>, _range: Range<usize>) -> Option<Hover> {
        let kind = node.green.kind;

        // Provide context-aware hover information
        let contents = match kind {
            ScssSyntaxKind::Selector => "### SCSS Selector\nDefines the target elements for styling.",
            ScssSyntaxKind::Property => "### SCSS Property\nDefines a style attribute.",
            ScssSyntaxKind::Variable => "### SCSS Variable\nDefines a reusable value.",
            _ => return None,
        };

        Some(Hover { contents: contents.to_string(), range: Some(node.span()) })
    }
}

/// Language service implementation for SCSS.
pub struct ScssLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: ScssHoverProvider,
}

impl<V: Vfs> ScssLanguageService<V> {
    /// Creates a new `ScssLanguageService`.
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: ScssHoverProvider }
    }
}

impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for ScssLanguageService<V> {
    type Lang = ScssLanguage;
    type Vfs = V;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }

    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }

    fn get_root(&self, _uri: &str) -> impl Future<Output = Option<RedNode<'_, ScssLanguage>>> + Send + '_ {
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
