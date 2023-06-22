use crate::language::RegexLanguage;
use core::range::Range;
use futures::Future;
use oak_core::tree::RedNode;
use oak_hover::{Hover, HoverProvider};
use oak_lsp::service::LanguageService;
use oak_vfs::Vfs;

#[allow(missing_docs)]
pub struct RegexHoverProvider;

impl HoverProvider<RegexLanguage> for RegexHoverProvider {
    fn hover(&self, _node: &RedNode<'_, RegexLanguage>, _range: Range<usize>) -> Option<Hover> {
        None
    }
}

#[allow(missing_docs)]
pub struct RegexLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: RegexHoverProvider,
}

impl<V: Vfs> RegexLanguageService<V> {
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: RegexHoverProvider }
    }
}

impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for RegexLanguageService<V> {
    type Lang = RegexLanguage;
    type Vfs = V;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }
    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }

    fn get_root(&self, uri: &str) -> impl Future<Output = Option<RedNode<'_, RegexLanguage>>> + Send + '_ {
        let source = self.vfs().get_source(uri);
        async move {
            let source = source?;
            let language = RegexLanguage::default();
            let parser = crate::parser::RegexParser::new(&language);
            let lexer = crate::lexer::RegexLexer::new(&language);
            let mut cache = oak_core::parser::session::ParseSession::<RegexLanguage>::default();
            let _output = oak_core::parser::parse(&parser, &lexer, &source, &[], &mut cache);
            None
        }
    }

    fn hover(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Option<oak_lsp::Hover>> + Send + '_ {
        let uri = uri.to_string();
        async move { self.with_root(&uri, |root| self.hover_provider.hover(&root, range).map(|h| oak_lsp::Hover { contents: h.contents, range: h.range })).await.flatten() }
    }
}
