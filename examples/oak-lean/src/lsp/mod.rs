#![doc = include_str!("readme.md")]
#[cfg(feature = "oak-highlight")]
pub mod highlighter;

use crate::language::LeanLanguage;
#[cfg(feature = "lsp")]
use oak_lsp::{LanguageService, MemoryVfs, WorkspaceManager};
#[cfg(feature = "lsp")]
pub struct LeanLanguageService {
    _config: LeanLanguage,
    vfs: MemoryVfs,
    workspace: WorkspaceManager,
}
#[cfg(feature = "lsp")]
impl LeanLanguageService {
    pub fn new(config: LeanLanguage) -> Self {
        Self { _config: config, vfs: MemoryVfs::default(), workspace: WorkspaceManager::default() }
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
}
