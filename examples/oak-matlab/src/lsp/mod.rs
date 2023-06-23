#![doc = include_str!("readme.md")]
#[cfg(feature = "oak-highlight")]
pub mod highlighter;

use crate::{MatlabLanguage, parser::element_type::MatlabElementType};
use core::range::Range;
use oak_core::tree::RedNode;
#[cfg(feature = "lsp")]
use {oak_hover::Hover, oak_hover::HoverProvider, oak_lsp::service::LanguageService, oak_vfs::Vfs, std::future::Future};
/// Hover provider implementation for MATLAB.
#[cfg(feature = "lsp")]
pub struct MatlabHoverProvider;
#[cfg(feature = "lsp")]
impl HoverProvider<MatlabLanguage> for MatlabHoverProvider {
    fn hover(&self, node: &RedNode<MatlabLanguage>, _range: Range<usize>) -> Option<oak_hover::Hover> {
        let kind = node.green.kind;
        let contents = match kind {
            MatlabElementType::FunctionDef => "### MATLAB Function\nDefines a reusable function block.",
            MatlabElementType::ClassDef => "### MATLAB Class\nDefines a class for object-oriented programming.",
            MatlabElementType::Script => "### MATLAB Script\nA file containing a sequence of MATLAB commands.",
            _ => return None,
        };
        Some(oak_hover::Hover { contents: contents.to_string(), range: Some(node.span()) })
    }
}
/// Language service implementation for MATLAB.
#[cfg(feature = "lsp")]
pub struct MatlabLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: MatlabHoverProvider,
}
impl<V: Vfs> MatlabLanguageService<V> {
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: MatlabHoverProvider }
    }
}
impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for MatlabLanguageService<V> {
    type Lang = MatlabLanguage;
    type Vfs = V;
    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }
    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }
    fn get_root(&self, _uri: &str) -> impl Future<Output = Option<RedNode<'_, MatlabLanguage>>> + Send + '_ {
        async move {
            // TODO: Implement proper caching and conversion to RedNode
            None
        }
    }
    fn hover(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Option<oak_lsp::types::Hover>> + Send + '_ {
        let uri = uri.to_string();
        async move {
            let hover = self.with_root(&uri, |root| self.hover_provider.hover(&root, range)).await.flatten()?;
            Some(oak_lsp::types::Hover { contents: hover.contents, range: hover.range })
        }
    }
}
