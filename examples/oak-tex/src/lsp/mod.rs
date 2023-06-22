use crate::language::TexLanguage;
use futures::Future;
use oak_core::tree::RedNode;
use oak_lsp::{service::LanguageService, workspace::WorkspaceManager};
use oak_vfs::{MemoryVfs, Vfs};

/// TeX 语言服务
pub struct TexLanguageService {
    vfs: MemoryVfs,
    workspace: WorkspaceManager,
}

impl TexLanguageService {
    /// 创建新的 TeX 语言服务
    pub fn new(vfs: MemoryVfs) -> Self {
        Self { vfs, workspace: WorkspaceManager::default() }
    }
}

impl LanguageService for TexLanguageService {
    type Lang = TexLanguage;
    type Vfs = MemoryVfs;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }

    fn workspace(&self) -> &WorkspaceManager {
        &self.workspace
    }

    fn get_root(&self, uri: &str) -> impl Future<Output = Option<RedNode<'_, TexLanguage>>> + Send + '_ {
        let source = self.vfs().get_source(uri);
        async move {
            let _source = source?;
            // TODO: 实现真正的解析逻辑
            None
        }
    }
}
