#![doc = include_str!("readme.md")]
#[cfg(feature = "oak-highlight")]
pub mod highlighter;

#[cfg(feature = "lsp")]
use {
    oak_lsp::{service::LanguageService, workspace::WorkspaceManager},
    oak_vfs::MemoryVfs,
};
#[cfg(feature = "oak-pretty-print")]
pub mod formatter;
use crate::language::TypstLanguage;
/// Typst language service.
#[cfg(feature = "lsp")]
pub struct TypstLanguageService {
    vfs: MemoryVfs,
    workspace: WorkspaceManager,
}
#[cfg(feature = "lsp")]
impl TypstLanguageService {
    pub fn new(vfs: MemoryVfs) -> Self {
        Self { vfs, workspace: WorkspaceManager::default() }
    }
}
#[cfg(feature = "lsp")]
impl LanguageService for TypstLanguageService {
    type Lang = TypstLanguage;
    type Vfs = MemoryVfs;
    fn vfs(&self) -> &MemoryVfs {
        &self.vfs
    }
    fn workspace(&self) -> &WorkspaceManager {
        &self.workspace
    }
}
