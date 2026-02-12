#![doc = include_str!("readme.md")]
use crate::language::TsvLanguage;
use oak_core::tree::RedNode;
use std::ops::Range;
#[cfg(feature = "lsp")]
use {futures::Future, oak_lsp::service::LanguageService, oak_lsp::types::Hover as LspHover, oak_vfs::Vfs};
#[cfg(feature = "lsp")]
/// Language service implementation for TSV.
pub struct TsvLanguageService<V: Vfs> {
    /// The virtual file system.
    vfs: V,
    /// The workspace manager.
    workspace: oak_lsp::workspace::WorkspaceManager,
}
impl<V: Vfs> TsvLanguageService<V> {
    /// Creates a new `TsvLanguageService`.
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::new() }
    }
}
impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for TsvLanguageService<V> {
    type Lang = TsvLanguage;
    type Vfs = V;
    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }
    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }
    fn get_root(&self, _uri: &str) -> impl Future<Output = Option<RedNode<'_, TsvLanguage>>> + Send + '_ {
        async move { None }
    }
    fn hover(&self, _uri: &str, _range: Range<usize>) -> impl Future<Output = Option<LspHover>> + Send + '_ {
        async move { None }
    }
}
