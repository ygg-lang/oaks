#![doc = include_str!("readme.md")]
#[cfg(feature = "oak-pretty-print")]
pub mod formatter;
#[cfg(feature = "oak-highlight")]
pub mod highlighter;

use crate::language::TailwindLanguage;

#[cfg(feature = "lsp")]
use {oak_lsp::LanguageService, oak_vfs::MemoryVfs};

#[cfg(feature = "lsp")]
/// Language service implementation for Tailwind CSS.
#[cfg(feature = "lsp")]
pub struct TailwindLanguageService {
    vfs: MemoryVfs,
    workspace: oak_lsp::workspace::WorkspaceManager,
}

#[cfg(feature = "lsp")]
impl TailwindLanguageService {
    /// Creates a new `TailwindLanguageService` with the given VFS.
    pub fn new(vfs: MemoryVfs) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default() }
    }
}

#[cfg(feature = "lsp")]
impl LanguageService for TailwindLanguageService {
    type Lang = TailwindLanguage;
    type Vfs = MemoryVfs;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }

    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }

    fn get_root(&self, _uri: &str) -> impl std::future::Future<Output = Option<oak_core::tree::RedNode<'_, TailwindLanguage>>> + Send + '_ {
        async move { None }
    }

    fn completion(&self, _uri: &str, _offset: usize) -> impl std::future::Future<Output = Vec<oak_lsp::types::CompletionItem>> + Send + '_ {
        async move { vec![] }
    }
}
