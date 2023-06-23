#![doc = include_str!("readme.md")]
#[cfg(feature = "oak-highlight")]
pub mod highlighter;

#[cfg(feature = "lsp")]
use {
    futures::Future,
    oak_hover::{Hover, HoverProvider},
    oak_lsp::service::LanguageService,
    oak_vfs::Vfs,
};
#[cfg(feature = "oak-pretty-print")]
pub mod formatter;
use crate::{language::AplLanguage, parser::AplElementType};
use core::range::Range;
use oak_core::tree::RedNode;
/// Hover provider implementation for APL.
#[cfg(feature = "lsp")]
pub struct AplHoverProvider;
#[cfg(feature = "lsp")]
impl HoverProvider<AplLanguage> for AplHoverProvider {
    fn hover(&self, node: &RedNode<AplLanguage>, _range: Range<usize>) -> Option<Hover> {
        let kind = node.green.kind;
        // Provide context-aware hover information
        let contents = match kind {
            AplElementType::Assignment => "### APL Assignment\nAssigns a value to a variable.",
            AplElementType::ArrayLiteral => "### APL Array\nA collection of values.",
            _ => return None,
        };
        Some(Hover { contents: contents.to_string(), range: Some(node.span()) })
    }
}
/// Language service implementation for APL.
#[cfg(feature = "lsp")]
pub struct AplLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: AplHoverProvider,
}
#[cfg(feature = "lsp")]
impl<V: Vfs> AplLanguageService<V> {
    /// Creates a new `AplLanguageService`.
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: AplHoverProvider }
    }
}
#[cfg(feature = "lsp")]
impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for AplLanguageService<V> {
    type Lang = AplLanguage;
    type Vfs = V;
    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }
    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }
    fn get_root(&self, uri: &str) -> impl Future<Output = Option<RedNode<'_, AplLanguage>>> + Send + '_ {
        let source = self.vfs().get_source(uri);
        async move {
            let source = source?;
            let language = AplLanguage::default();
            let parser = crate::parser::AplParser::new(&language);
            let lexer = crate::lexer::AplLexer::new(&language);
            let mut cache = Box::new(oak_core::parser::session::ParseSession::<AplLanguage>::default());
            let cache_ptr: *mut oak_core::parser::session::ParseSession<AplLanguage> = &mut *cache;
            let parse_out = oak_core::parser::parse(&parser, &lexer, &source, &[], unsafe { &mut *cache_ptr });
            let green = parse_out.result.ok()?;
            let _leaked_cache = Box::leak(cache);
            let green_static: &'static oak_core::GreenNode<AplLanguage> = unsafe { std::mem::transmute(green) };
            Some(RedNode::new(green_static, 0))
        }
    }
    fn hover(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Option<oak_lsp::Hover>> + Send + '_ {
        let uri = uri.to_string();
        async move { self.with_root(&uri, |root| self.hover_provider.hover(&root, range).map(|h| oak_lsp::Hover { contents: h.contents, range: h.range })).await.flatten() }
    }
}
