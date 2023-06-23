#![doc = include_str!("readme.md")]
#[cfg(feature = "oak-highlight")]
pub mod highlighter;

#[cfg(feature = "lsp")]
use futures::Future;
#[cfg(feature = "lsp")]
use oak_hover::{Hover, HoverProvider};
#[cfg(feature = "lsp")]
use oak_lsp::service::LanguageService;
#[cfg(feature = "lsp")]
use oak_vfs::Vfs;
#[cfg(feature = "oak-pretty-print")]
pub mod formatter;
use crate::{language::RLanguage, parser::element_type::RElementType};
use core::range::Range;
use oak_core::tree::RedNode;
#[cfg(feature = "lsp")]
pub struct RHoverProvider;
#[cfg(feature = "lsp")]
impl HoverProvider<RLanguage> for RHoverProvider {
    fn hover(&self, node: &RedNode<RLanguage>, _range: Range<usize>) -> Option<Hover> {
        let kind = node.green.kind;
        let contents = match kind {
            RElementType::FunctionDefinition => "### R Function\nDefines a callable block of code.",
            _ => return None,
        };
        Some(Hover { contents: contents.to_string(), range: Some(node.span()) })
    }
}
#[cfg(feature = "lsp")]
pub struct RLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: RHoverProvider,
}
impl<V: Vfs> RLanguageService<V> {
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: RHoverProvider }
    }
}
impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for RLanguageService<V> {
    type Lang = RLanguage;
    type Vfs = V;
    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }
    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }
    fn get_root(&self, uri: &str) -> impl Future<Output = Option<RedNode<'_, RLanguage>>> + Send + '_ {
        let source = self.vfs().get_source(uri);
        async move {
            let source = source?;
            let language = RLanguage::default();
            let parser = crate::parser::RParser::new(&language);
            let lexer = crate::lexer::RLexer::new(&language);
            let mut cache = oak_core::parser::session::ParseSession::<RLanguage>::default();
            let _output = oak_core::parser::parse(&parser, &lexer, &source, &[], &mut cache);
            None
        }
    }
    fn hover(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Option<oak_lsp::Hover>> + Send + '_ {
        let uri = uri.to_string();
        async move { self.with_root(&uri, |root| self.hover_provider.hover(&root, range).map(|h| oak_lsp::Hover { contents: h.contents, range: h.range })).await.flatten() }
    }
}
