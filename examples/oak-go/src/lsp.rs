//! Go 语言 LSP 服务

use crate::language::GoLanguage;
use oak_core::tree::RedNode;
use oak_lsp::LanguageService;
use oak_vfs::MemoryVfs;
use std::future::Future;

/// Go 语言服务
pub struct GoLanguageService {
    vfs: MemoryVfs,
    workspace: oak_lsp::workspace::WorkspaceManager,
}

impl GoLanguageService {
    /// 创建一个新的 GoLanguageService
    pub fn new(vfs: MemoryVfs) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default() }
    }
}

impl LanguageService for GoLanguageService {
    type Lang = GoLanguage;
    type Vfs = MemoryVfs;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }

    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }

    fn get_root(&self, _uri: &str) -> impl Future<Output = Option<RedNode<'_, Self::Lang>>> + Send + '_ {
        async { None }
    }
}
