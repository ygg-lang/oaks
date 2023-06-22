use crate::language::TypeScriptLanguage;
use oak_lsp::LanguageService;
use oak_vfs::MemoryVfs;

/// TypeScript 语言服务
pub struct TypeScriptLanguageService {
    vfs: MemoryVfs,
    workspace: oak_lsp::workspace::WorkspaceManager,
}

impl TypeScriptLanguageService {
    pub fn new(vfs: MemoryVfs) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default() }
    }
}

impl LanguageService for TypeScriptLanguageService {
    type Lang = TypeScriptLanguage;
    type Vfs = MemoryVfs;

    fn vfs(&self) -> &MemoryVfs {
        &self.vfs
    }

    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }
}
