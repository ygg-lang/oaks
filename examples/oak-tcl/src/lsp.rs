use crate::{TclLanguage, kind::TclSyntaxKind};
use core::range::Range;
use futures::Future;
use oak_core::tree::RedNode;
use oak_hover::{Hover, HoverProvider};
use oak_lsp::service::LanguageService;
use oak_vfs::Vfs;

/// Tcl 的 Hover 提供者实现
pub struct TclHoverProvider;

impl HoverProvider<TclLanguage> for TclHoverProvider {
    fn hover(&self, node: &RedNode<TclLanguage>, _range: Range<usize>) -> Option<Hover> {
        let kind = node.green.kind;

        let contents = match kind {
            TclSyntaxKind::Proc => "### Tcl Procedure\nDefines a command.",
            TclSyntaxKind::Set => "### Tcl Set\nSets the value of a variable.",
            _ => return None,
        };

        Some(Hover { contents: contents.to_string(), range: Some(node.span()) })
    }
}

/// Tcl 的语言服务实现
pub struct TclLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: TclHoverProvider,
}

impl<V: Vfs> TclLanguageService<V> {
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: TclHoverProvider }
    }
}

impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for TclLanguageService<V> {
    type Lang = TclLanguage;
    type Vfs = V;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }

    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }

    fn get_root(&self, _uri: &str) -> impl Future<Output = Option<RedNode<'_, TclLanguage>>> + Send + '_ {
        async move { None }
    }

    fn hover(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Option<oak_lsp::Hover>> + Send + '_ {
        let uri = uri.to_string();
        async move { self.with_root(&uri, |root| self.hover_provider.hover(&root, range).map(|h| oak_lsp::Hover { contents: h.contents, range: h.range })).await.flatten() }
    }
}
