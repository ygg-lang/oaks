use crate::language::JavadocLanguage;
use oak_core::tree::RedNode;
use oak_lsp::service::LanguageService;
use oak_vfs::Vfs;
use std::future::Future;

pub struct JavadocLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
}

impl<V: Vfs> JavadocLanguageService<V> {
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default() }
    }
}

impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for JavadocLanguageService<V> {
    type Lang = JavadocLanguage;
    type Vfs = V;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }

    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }

    fn get_root(&self, uri: &str) -> impl Future<Output = Option<RedNode<'_, JavadocLanguage>>> + Send + '_ {
        let source = self.vfs().get_source(uri);
        async move {
            let source = source?;
            let language = JavadocLanguage::default();
            let parser = crate::parser::JavadocParser::new(&language);
            let lexer = crate::lexer::JavadocLexer::new(&language);

            let mut cache = oak_core::parser::session::ParseSession::<JavadocLanguage>::default();
            let _output = oak_core::parser::parse(&parser, &lexer, &source, &[], &mut cache);

            // FIXME: Store the tree in workspace and return it
            // For now, return None to satisfy compiler lifetimes
            None
        }
    }
}
