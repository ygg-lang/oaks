#![doc = include_str!("readme.md")]
#[cfg(feature = "oak-highlight")]
pub mod highlighter;

use crate::JuliaLanguage;
#[cfg(feature = "lsp")]
use {futures::Future, oak_lsp::service::LanguageService, oak_vfs::Vfs};
#[cfg(feature = "lsp")]
pub struct JuliaLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
}
impl<V: Vfs> JuliaLanguageService<V> {
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default() }
    }
}
impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for JuliaLanguageService<V> {
    type Lang = JuliaLanguage;
    type Vfs = V;
    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }
    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }
    fn get_root(&self, _uri: &str) -> impl Future<Output = Option<oak_core::tree::RedNode<'_, Self::Lang>>> + Send + '_ {
        async move { None }
    }
}
