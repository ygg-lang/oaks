#![doc = include_str!("readme.md")]
#[cfg(feature = "lsp")]
use {
    oak_lsp::service::LanguageService,
    oak_vfs::Vfs,
    futures::Future,
    oak_lsp::types::Hover as LspHover,
};
use crate::language::MojoLanguage;
use oak_core::tree::RedNode;
use core::range::Range;
#[cfg(feature = "lsp")]
pub struct MojoLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
}
impl<V: Vfs> MojoLanguageService<V> {
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::new() }
    }
}
impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for MojoLanguageService<V> {
    type Lang = MojoLanguage;
    type Vfs = V;
    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }
    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }
    fn get_root(&self, _uri: &str) -> impl Future<Output = Option<RedNode<'_, MojoLanguage>>> + Send + '_ {
        async move { None }
    }
    fn hover(&self, _uri: &str, _range: Range<usize>) -> impl Future<Output = Option<LspHover>> + Send + '_ {
        async move { None }
    }
}