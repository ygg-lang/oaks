use crate::language::JavaScriptLanguage;
use oak_core::tree::RedNode;
use oak_lsp::service::LanguageService;
use oak_vfs::Vfs;
use std::future::Future;

pub struct JavaScriptLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
}

impl<V: Vfs> JavaScriptLanguageService<V> {
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default() }
    }
}

impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for JavaScriptLanguageService<V> {
    type Lang = JavaScriptLanguage;
    type Vfs = V;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }

    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }

    fn get_root(&self, uri: &str) -> impl Future<Output = Option<RedNode<'_, JavaScriptLanguage>>> + Send + '_ {
        let source = self.vfs().get_source(uri);
        async move {
            let source = source?;
            let language = JavaScriptLanguage::default();
            let parser = crate::parser::JavaScriptParser::new(language);
            let lexer = crate::lexer::JavaScriptLexer::new(&language);

            let mut cache = oak_core::parser::session::ParseSession::<JavaScriptLanguage>::new(1024);
            let _output = oak_core::parser::parse(&parser, &lexer, &source, &[], &mut cache);

            // TODO: In a real implementation, you would convert GreenNode to RedNode properly
            None
        }
    }
}
