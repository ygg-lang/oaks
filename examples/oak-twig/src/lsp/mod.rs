use crate::language::TwigLanguage;
use oak_lsp::LanguageService;
use oak_vfs::MemoryVfs;

/// Twig 语言服务
pub struct TwigLanguageService {
    vfs: MemoryVfs,
    workspace: oak_lsp::workspace::WorkspaceManager,
}

impl TwigLanguageService {
    /// 创建新的语言服务
    pub fn new(vfs: MemoryVfs) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default() }
    }
}

impl LanguageService for TwigLanguageService {
    type Lang = TwigLanguage;
    type Vfs = MemoryVfs;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }

    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }

    fn get_root(&self, _uri: &str) -> impl std::future::Future<Output = Option<oak_core::tree::RedNode<'_, TwigLanguage>>> + Send + '_ {
        async move { None }
    }
}
