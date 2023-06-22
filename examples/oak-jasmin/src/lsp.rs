use crate::language::JasminLanguage;
use core::range::Range;
use futures::Future;
use oak_core::tree::RedNode;
use oak_lsp::service::LanguageService;
use oak_vfs::Vfs;

pub struct JasminLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
}

impl<V: Vfs> JasminLanguageService<V> {
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default() }
    }
}

impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for JasminLanguageService<V> {
    type Lang = JasminLanguage;
    type Vfs = V;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }

    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }

    fn get_root(&self, uri: &str) -> impl Future<Output = Option<RedNode<'_, JasminLanguage>>> + Send + '_ {
        let source = self.vfs().get_source(uri);
        async move {
            let _source = source?;
            None
        }
    }

    fn hover(&self, _uri: &str, _range: Range<usize>) -> impl Future<Output = Option<oak_lsp::Hover>> + Send + '_ {
        async move { None }
    }
}
