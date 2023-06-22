use crate::language::KotlinLanguage;
use oak_lsp::{LanguageService, WorkspaceManager};
use oak_vfs::MemoryVfs;

pub struct KotlinLanguageService {
    _language: KotlinLanguage,
    vfs: MemoryVfs,
    workspace: WorkspaceManager,
}

impl KotlinLanguageService {
    pub fn new(language: KotlinLanguage) -> Self {
        Self { _language: language, vfs: MemoryVfs::default(), workspace: WorkspaceManager::default() }
    }
}

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
