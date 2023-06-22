use crate::language::LeanLanguage;
use oak_lsp::{LanguageService, MemoryVfs, WorkspaceManager};

pub struct LeanLanguageService {
    _config: LeanLanguage,
    vfs: MemoryVfs,
    workspace: WorkspaceManager,
}

impl LeanLanguageService {
    pub fn new(config: LeanLanguage) -> Self {
        Self { _config: config, vfs: MemoryVfs::default(), workspace: WorkspaceManager::default() }
    }
}

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
