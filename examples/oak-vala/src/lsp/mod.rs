use crate::language::ValaLanguage;
use oak_lsp::LanguageService;
use oak_vfs::MemoryVfs;

/// Vala 语言服务
pub struct ValaLanguageService {
    vfs: MemoryVfs,
    workspace: oak_lsp::workspace::WorkspaceManager,
}

impl ValaLanguageService {
    pub fn new(vfs: MemoryVfs) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default() }
    }
}

impl LanguageService for ValaLanguageService {
    type Lang = ValaLanguage;
    type Vfs = MemoryVfs;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }

    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }
}
