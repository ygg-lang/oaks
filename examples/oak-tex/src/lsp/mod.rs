#![doc = include_str!("readme.md")]
#[cfg(feature = "oak-highlight")]
pub mod highlighter;

#[cfg(feature = "lsp")]
use {
    futures::Future,
    oak_lsp::{service::LanguageService, workspace::WorkspaceManager},
    oak_vfs::{MemoryVfs, Vfs},
};
#[cfg(feature = "oak-pretty-print")]
pub mod formatter;
use crate::language::TexLanguage;
use oak_core::tree::RedNode;
/// TeX language service.
#[cfg(feature = "lsp")]
pub struct TexLanguageService {
    vfs: MemoryVfs,
    workspace: WorkspaceManager,
}
#[cfg(feature = "lsp")]
impl TexLanguageService {
    /// Creates a new TeX language service.
    pub fn new(vfs: MemoryVfs) -> Self {
        Self { vfs, workspace: WorkspaceManager::default() }
    }
}
#[cfg(feature = "lsp")]
impl LanguageService for TexLanguageService {
    type Lang = TexLanguage;
    type Vfs = MemoryVfs;
    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }
    fn workspace(&self) -> &WorkspaceManager {
        &self.workspace
    }
    fn get_root(&self, uri: &str) -> impl Future<Output = Option<RedNode<'_, TexLanguage>>> + Send + '_ {
        let source = self.vfs().get_source(uri);
        async move {
            let _source = source?;
            // TODO: 实现真正的解析逻辑
            None
        }
    }
}
