#![doc = include_str!("readme.md")]
#[cfg(feature = "oak-highlight")]
pub mod highlighter;

#[cfg(feature = "lsp")]
use {oak_lsp::LanguageService, oak_vfs::MemoryVfs, std::future::Future};
#[cfg(feature = "oak-pretty-print")]
pub mod formatter;
/// Go language LSP service.
use crate::language::GoLanguage;
use oak_core::tree::RedNode;
/// Go language service.
#[cfg(feature = "lsp")]
pub struct GoLanguageService {
    vfs: MemoryVfs,
    workspace: oak_lsp::workspace::WorkspaceManager,
}
#[cfg(feature = "lsp")]
impl GoLanguageService {
    /// Creates a new `GoLanguageService`.
    pub fn new(vfs: MemoryVfs) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default() }
    }
}
#[cfg(feature = "lsp")]
impl LanguageService for GoLanguageService {
    type Lang = GoLanguage;
    type Vfs = MemoryVfs;
    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }
    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }
    fn get_root(&self, _uri: &str) -> impl Future<Output = Option<RedNode<'_, Self::Lang>>> + Send + '_ {
        async { None }
    }
}
