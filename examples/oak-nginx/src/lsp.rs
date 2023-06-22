use crate::{NginxLanguage, kind::NginxSyntaxKind};
use core::range::Range;
use oak_core::tree::RedNode;
use oak_hover::HoverProvider;
use oak_lsp::{service::LanguageService, types::Hover};
use oak_vfs::Vfs;

use std::future::Future;

/// Hover provider implementation for Nginx.
pub struct NginxHoverProvider;

impl HoverProvider<NginxLanguage> for NginxHoverProvider {
    fn hover(&self, root: &RedNode<'_, NginxLanguage>, _range: Range<usize>) -> Option<oak_hover::Hover> {
        let kind = root.green.kind;

        let contents = match kind {
            NginxSyntaxKind::Directive => "### Nginx Directive\nA configuration instruction for the Nginx server.",
            NginxSyntaxKind::Block => "### Nginx Block\nA group of directives enclosed in braces.",
            NginxSyntaxKind::Root => "### Nginx Configuration\nThe root of an Nginx configuration file.",
            _ => return None,
        };

        Some(oak_hover::Hover { contents: contents.to_string(), range: Some(root.span()) })
    }
}

/// Language service implementation for Nginx.
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
