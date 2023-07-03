#[cfg(feature = "lsp")]
pub mod highlighter;

#[cfg(feature = "lsp")]
pub mod formatter;

#[cfg(feature = "lsp")]
pub use highlighter::GlobHighlighter;

#[cfg(feature = "lsp")]
pub use formatter::GlobFormatter;

#[cfg(feature = "lsp")]
use oak_lsp::service::LanguageService;
use oak_vfs::Vfs;

#[cfg(feature = "lsp")]
/// Language service for glob patterns.
pub struct GlobLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
}

#[cfg(feature = "lsp")]
impl<V: Vfs> GlobLanguageService<V> {
    /// Creates a new `GlobLanguageService`.
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default() }
    }
}

#[cfg(feature = "lsp")]
impl<V: Vfs> LanguageService for GlobLanguageService<V> {
    type Lang = crate::language::GlobLanguage;
    type Vfs = V;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }

    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }

    fn get_root(&self, uri: &str) -> impl std::future::Future<Output = Option<oak_core::tree::RedNode<'_, Self::Lang>>> + Send + '_ {
        let source = self.vfs().get_source(uri);
        async move {
            let _source = source?;
            // TODO: Implement actual parsing here if needed
            None
        }
    }
}

#[cfg(feature = "lsp")]
impl Default for GlobLanguageService<oak_vfs::DiskVfs> {
    fn default() -> Self {
        Self::new(oak_vfs::DiskVfs::default())
    }
}
