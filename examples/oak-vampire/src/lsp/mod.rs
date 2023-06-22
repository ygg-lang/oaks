use crate::language::VampireLanguage;
use oak_lsp::LanguageService;
use oak_vfs::MemoryVfs;

/// Vampire 语言服务
pub struct VampireLanguageService {
    vfs: MemoryVfs,
    workspace: oak_lsp::workspace::WorkspaceManager,
}

impl VampireLanguageService {
    pub fn new(vfs: MemoryVfs) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default() }
    }
}

impl LanguageService for VampireLanguageService {
    type Lang = VampireLanguage;
    type Vfs = MemoryVfs;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }

    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }
}
