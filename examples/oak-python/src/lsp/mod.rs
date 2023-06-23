#![doc = include_str!("readme.md")]
#[cfg(feature = "oak-highlight")]
pub mod highlighter;

use crate::{language::PythonLanguage, parser::element_type::PythonElementType};
use core::range::Range;
use oak_core::tree::RedNode;
#[cfg(feature = "lsp")]
use {
    futures::Future,
    oak_hover::{Hover as HoverInfo, HoverProvider},
    oak_lsp::{service::LanguageService, types::Hover as LspHover},
    oak_vfs::Vfs,
};
/// Hover provider for Python.
#[cfg(feature = "lsp")]
pub struct PythonHoverProvider;
#[cfg(feature = "lsp")]
impl HoverProvider<PythonLanguage> for PythonHoverProvider {
    fn hover(&self, node: &RedNode<PythonLanguage>, _range: Range<usize>) -> Option<HoverInfo> {
        let kind = node.green.kind;
        let contents = match kind {
            PythonElementType::FunctionDef => "### Function Definition\nA standard Python function defined with `def`.",
            PythonElementType::ClassDef => "### Class Definition\nA standard Python class defined with `class`.",
            PythonElementType::Import | PythonElementType::ImportFrom => "### Import Statement\nImports modules or specific symbols.",
            _ => return None,
        };
        Some(HoverInfo { contents: contents.to_string(), range: Some(node.span()) })
    }
}
/// Language service for Python.
#[cfg(feature = "lsp")]
pub struct PythonLanguageService<V: Vfs> {
    /// Reference to the virtual file system.
    vfs: V,
    /// Reference to the workspace manager.
    workspace: oak_lsp::workspace::WorkspaceManager,
    /// Reference to the hover provider.
    hover_provider: PythonHoverProvider,
}

impl<V: Vfs> PythonLanguageService<V> {
    /// Creates a new Python language service.
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
