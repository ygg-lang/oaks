#![doc = include_str!("readme.md")]
#[cfg(feature = "oak-highlight")]
pub mod highlighter;

use crate::language::JavaLanguage;
use oak_core::tree::RedNode;
#[cfg(feature = "lsp")]
use {oak_lsp::LanguageService, oak_vfs::Vfs, std::future::Future};
#[cfg(feature = "lsp")]
pub struct JavaLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
}
impl<V: Vfs> JavaLanguageService<V> {
    pub fn new(vfs: V, _language: JavaLanguage) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default() }
    }
}
impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for JavaLanguageService<V> {
    type Lang = JavaLanguage;
    type Vfs = V;
    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }
    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }
    fn get_root(&self, _uri: &str) -> impl Future<Output = Option<RedNode<'_, JavaLanguage>>> + Send + '_ {
        async move { None }
    }
}
