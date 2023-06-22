use crate::language::LLvmLanguage;
use core::range::Range;
use futures::Future;
use oak_core::tree::RedNode;
use oak_hover::{Hover, HoverProvider};
use oak_lsp::service::LanguageService;
use oak_vfs::Vfs;

/// Hover provider implementation for LLVM IR.
pub struct LlirHoverProvider;

impl HoverProvider<LLvmLanguage> for LlirHoverProvider {
    fn hover(&self, node: &RedNode<LLvmLanguage>, _range: Range<usize>) -> Option<Hover> {
        let _kind = node.green.kind;
        // TODO: Provide context-aware hover information
        None
    }
}

/// Language service implementation for LLVM IR.
pub struct LlirLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: LlirHoverProvider,
}

impl<V: Vfs> LlirLanguageService<V> {
    /// Creates a new `LlirLanguageService`.
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: LlirHoverProvider }
    }
}

impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for LlirLanguageService<V> {
    type Lang = LLvmLanguage;
    type Vfs = V;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }

    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }

    fn get_root(&self, uri: &str) -> impl Future<Output = Option<RedNode<'_, LLvmLanguage>>> + Send + '_ {
        let source = self.vfs().get_source(uri);
        async move {
            let source = source?;
            let language = LLvmLanguage::default();
            let parser = crate::parser::LlirParser::new(&language);
            let lexer = crate::lexer::LlvmLexer::new(&language);

            let mut cache = oak_core::parser::session::ParseSession::<LLvmLanguage>::default();
            let _output = oak_core::parser::parse(&parser, &lexer, &source, &[], &mut cache);
            None
        }
    }

    fn hover(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Option<oak_lsp::Hover>> + Send + '_ {
        let uri = uri.to_string();
        async move { self.with_root(&uri, |root| self.hover_provider.hover(&root, range).map(|h| oak_lsp::Hover { contents: h.contents, range: h.range })).await.flatten() }
    }
}
