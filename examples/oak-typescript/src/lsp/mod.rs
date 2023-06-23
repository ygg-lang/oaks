#![doc = include_str!("readme.md")]
#[cfg(feature = "oak-highlight")]
pub mod highlighter;

#[cfg(feature = "lsp")]
use {
    futures::Future,
    oak_lsp::service::LanguageService,
    oak_lsp::types::Hover as LspHover,
    oak_vfs::{MemoryVfs, Vfs},
};

#[cfg(feature = "oak-pretty-print")]
pub mod formatter;
use crate::language::TypeScriptLanguage;
use core::range::Range;
use oak_core::tree::RedNode;

/// TypeScript language service.
#[cfg(feature = "lsp")]
pub struct TypeScriptLanguageService<V: Vfs = MemoryVfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
}

#[cfg(feature = "lsp")]
impl<V: Vfs> TypeScriptLanguageService<V> {
    /// Creates a new `TypeScriptLanguageService`.
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default() }
    }
}

#[cfg(feature = "lsp")]
impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for TypeScriptLanguageService<V> {
    type Lang = TypeScriptLanguage;
    type Vfs = V;
    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }
    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }
    fn get_root(&self, _uri: &str) -> impl Future<Output = Option<RedNode<'_, TypeScriptLanguage>>> + Send + '_ {
        async move { None }
    }
    fn hover(&self, _uri: &str, _range: Range<usize>) -> impl Future<Output = Option<LspHover>> + Send + '_ {
        async move { None }
    }
}
