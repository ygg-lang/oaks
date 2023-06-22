use crate::JasmLanguage;
use core::range::Range;
use futures::Future;
use oak_core::tree::RedNode;
use oak_hover::{Hover, HoverProvider};
use oak_lsp::service::LanguageService;
use oak_vfs::Vfs;

pub struct JasmHoverProvider;

impl HoverProvider<JasmLanguage> for JasmHoverProvider {
    fn hover(&self, node: &RedNode<JasmLanguage>, _range: Range<usize>) -> Option<Hover> {
        let kind = node.green.kind;
        let contents = format!("### JASM Node\nKind: {:?}", kind);
        Some(Hover { contents, range: Some(node.span()) })
    }
}

pub struct JasmLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: JasmHoverProvider,
}

impl<V: Vfs> JasmLanguageService<V> {
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: JasmHoverProvider }
    }
}

impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for JasmLanguageService<V> {
    type Lang = JasmLanguage;
    type Vfs = V;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }

    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }

    fn get_root(&self, uri: &str) -> impl Future<Output = Option<RedNode<'_, JasmLanguage>>> + Send + '_ {
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
