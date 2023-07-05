#![doc = include_str!("readme.md")]
#[cfg(feature = "oak-highlight")]
pub mod highlighter;

use crate::language::LeanLanguage;
#[cfg(feature = "lsp")]
use oak_lsp::{LanguageService, MemoryVfs, WorkspaceManager};
#[cfg(feature = "lsp")]
pub struct LeanLanguageService {
    config: LeanLanguage,
    vfs: MemoryVfs,
    workspace: WorkspaceManager,
}
#[cfg(feature = "lsp")]
impl LeanLanguageService {
    pub fn new(config: LeanLanguage) -> Self {
        Self { config, vfs: MemoryVfs::default(), workspace: WorkspaceManager::default() }
    }
}
#[cfg(feature = "lsp")]
impl LanguageService for LeanLanguageService {
    type Lang = LeanLanguage;
    type Vfs = MemoryVfs;
    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }
    fn workspace(&self) -> &WorkspaceManager {
        &self.workspace
    }
    fn with_root<R, F>(&self, _uri: &str, _f: F) -> impl std::future::Future<Output = Option<R>> + Send
    where
        R: Send,
        F: FnOnce(oak_core::tree::RedNode<'_, Self::Lang>) -> R + Send,
    {
        async { None }
    }
}
