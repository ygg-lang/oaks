#![doc = include_str!("readme.md")]
#[cfg(feature = "oak-highlight")]
pub mod highlighter;

use crate::{HaskellLanguage, lexer::token_type::HaskellTokenType, parser::element_type::HaskellElementType};
use core::range::Range;
use oak_core::tree::RedNode;
#[cfg(feature = "lsp")]
use {
    futures::Future,
    oak_hover::{Hover, HoverProvider},
    oak_lsp::service::LanguageService,
    oak_vfs::Vfs,
};
/// Hover provider implementation for Haskell.
#[cfg(feature = "lsp")]
pub struct HaskellHoverProvider;
#[cfg(feature = "lsp")]
impl HoverProvider<HaskellLanguage> for HaskellHoverProvider {
    fn hover(&self, node: &RedNode<HaskellLanguage>, _range: Range<usize>) -> Option<Hover> {
        let kind = node.green.kind;
        // Provide context-aware hover information
        let contents = match kind {
            HaskellElementType::Function => "### Haskell Function\nDefines a transformation from inputs to outputs.",
            HaskellElementType::DataDeclaration => "### Haskell Data Type\nDefines a new algebraic data type.",
            HaskellElementType::ModuleDeclaration => "### Haskell Module\nOrganizes Haskell code into namespaces.",
            _ => return None,
        };
        Some(Hover { contents: contents.to_string(), range: Some(node.span()) })
    }
}
/// Language service implementation for Haskell.
#[cfg(feature = "lsp")]
pub struct HaskellLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: HaskellHoverProvider,
}
impl<V: Vfs> HaskellLanguageService<V> {
    /// Creates a new `HaskellLanguageService`.
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: HaskellHoverProvider }
    }
}
impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for HaskellLanguageService<V> {
    type Lang = HaskellLanguage;
    type Vfs = V;
    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }
    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }
    fn get_root(&self, _uri: &str) -> impl Future<Output = Option<RedNode<'_, HaskellLanguage>>> + Send + '_ {
        async move {
            // TODO: Implement proper caching and conversion to RedNode
            None
        }
    }
    fn hover(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Option<oak_lsp::Hover>> + Send + '_ {
        let uri = uri.to_string();
        async move { self.with_root(&uri, |root| self.hover_provider.hover(&root, range).map(|h| oak_lsp::Hover { contents: h.contents, range: h.range })).await.flatten() }
    }
}
