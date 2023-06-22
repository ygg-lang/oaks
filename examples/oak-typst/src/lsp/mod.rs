use crate::language::TypstLanguage;
use oak_lsp::{service::LanguageService, workspace::WorkspaceManager};
use oak_vfs::MemoryVfs;

/// Typst 语言服务
pub struct TypstLanguageService {
    vfs: MemoryVfs,
    workspace: WorkspaceManager,
}

impl TypstLanguageService {
    pub fn new(vfs: MemoryVfs) -> Self {
        Self { vfs, workspace: WorkspaceManager::default() }
    }
}

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
