use crate::{kind::PythonSyntaxKind, language::PythonLanguage};
use core::range::Range;
use futures::Future;
use oak_core::tree::RedNode;
use oak_hover::{Hover as HoverInfo, HoverProvider};
use oak_lsp::{service::LanguageService, types::Hover as LspHover};
use oak_vfs::Vfs;

/// Hover provider for Python.
pub struct PythonHoverProvider;

impl HoverProvider<PythonLanguage> for PythonHoverProvider {
    fn hover(&self, node: &RedNode<PythonLanguage>, _range: Range<usize>) -> Option<HoverInfo> {
        let kind = node.green.kind;

        let contents = match kind {
            PythonSyntaxKind::FunctionDef => "### Function Definition\nA standard Python function defined with `def`.",
            PythonSyntaxKind::ClassDef => "### Class Definition\nA standard Python class defined with `class`.",
            PythonSyntaxKind::Import | PythonSyntaxKind::ImportFrom => "### Import Statement\nImports modules or specific symbols.",
            _ => return None,
        };

        Some(HoverInfo { contents: contents.to_string(), range: Some(node.span()) })
    }
}

pub struct PythonLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: PythonHoverProvider,
}

impl<V: Vfs> PythonLanguageService<V> {
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::new(), hover_provider: PythonHoverProvider }
    }
}

impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for PythonLanguageService<V> {
    type Lang = PythonLanguage;
    type Vfs = V;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }

    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }

    fn get_root(&self, uri: &str) -> impl Future<Output = Option<RedNode<'_, PythonLanguage>>> + Send + '_ {
        let _source = self.get_source(uri);
        async move {
            // NOTE: Python parser integration here
            None
        }
    }

    fn hover(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Option<LspHover>> + Send + '_ {
        let uri = uri.to_string();
        async move { self.with_root(&uri, |root| self.hover_provider.hover(&root, range).map(|h| LspHover { contents: h.contents, range: h.range })).await.flatten() }
    }
}
