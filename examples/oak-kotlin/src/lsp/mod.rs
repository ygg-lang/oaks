#![doc = include_str!("readme.md")]
#[cfg(feature = "oak-highlight")]
pub mod highlighter;

use crate::language::KotlinLanguage;
#[cfg(feature = "lsp")]
use {
    oak_lsp::{LanguageService, WorkspaceManager},
    oak_vfs::MemoryVfs,
};
#[cfg(feature = "lsp")]
pub struct KotlinLanguageService {
    _language: KotlinLanguage,
    vfs: MemoryVfs,
    workspace: WorkspaceManager,
}
#[cfg(feature = "lsp")]
impl KotlinLanguageService {
    pub fn new(language: KotlinLanguage) -> Self {
        Self { _language: language, vfs: MemoryVfs::default(), workspace: WorkspaceManager::default() }
    }
}
#[cfg(feature = "lsp")]
impl LanguageService for KotlinLanguageService {
    type Lang = KotlinLanguage;
    type Vfs = MemoryVfs;
    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }
    fn workspace(&self) -> &WorkspaceManager {
        &self.workspace
    }
}
