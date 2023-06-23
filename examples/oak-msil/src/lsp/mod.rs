#![doc = include_str!("readme.md")]
#[cfg(feature = "oak-highlight")]
pub mod highlighter;

use crate::{MsilLanguage, parser::element_type::MsilElementType};
use core::range::Range;
use oak_core::tree::RedNode;
#[cfg(feature = "lsp")]
use {
    oak_hover::HoverProvider,
    oak_lsp::{service::LanguageService, types::Hover},
    oak_vfs::Vfs,
    std::future::Future,
};
/// Hover provider implementation for MSIL.
#[cfg(feature = "lsp")]
pub struct MsilHoverProvider;
#[cfg(feature = "lsp")]
impl HoverProvider<MsilLanguage> for MsilHoverProvider {
    fn hover(&self, node: &RedNode<MsilLanguage>, _range: Range<usize>) -> Option<oak_hover::Hover> {
        let kind = node.green.kind;
        let contents = match kind {
            MsilElementType::Method => "### MSIL Method\nDefines a method in MSIL.",
            MsilElementType::Class => "### MSIL Class\nDefines a class in MSIL.",
            MsilElementType::Assembly => "### MSIL Assembly\nDefines an assembly in MSIL.",
            MsilElementType::Instruction => "### MSIL Instruction\nA single MSIL instruction.",
            _ => return None,
        };
        Some(oak_hover::Hover { contents: contents.to_string(), range: Some(node.span()) })
    }
}
/// Language service implementation for MSIL.
#[cfg(feature = "lsp")]
pub struct MsilLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: MsilHoverProvider,
}
#[cfg(feature = "lsp")]
impl<V: Vfs> MsilLanguageService<V> {
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: MsilHoverProvider }
    }
}
#[cfg(feature = "lsp")]
impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for MsilLanguageService<V> {
    type Lang = MsilLanguage;
    type Vfs = V;
    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }
    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }
    fn get_root(&self, uri: &str) -> impl Future<Output = Option<RedNode<'_, MsilLanguage>>> + Send + '_ {
        let source = self.vfs().get_source(uri);
        async move {
            let source = source?;
            let language = MsilLanguage::new();
            let parser = crate::parser::MsilParser::new(&language);
            let lexer = crate::lexer::MsilLexer::new(&language);
            let mut cache = Box::new(oak_core::parser::session::ParseSession::<MsilLanguage>::default());
            let cache_ptr: *mut oak_core::parser::session::ParseSession<MsilLanguage> = &mut *cache;
            let parse_out = oak_core::parser::parse(&parser, &lexer, &source, &[], unsafe { &mut *cache_ptr });
            let green = parse_out.result.ok()?;
            let _leaked_cache = Box::leak(cache);
            let green_static: &'static oak_core::GreenNode<MsilLanguage> = unsafe { std::mem::transmute(green) };
            Some(RedNode::new(green_static, 0))
        }
    }
    fn hover(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Option<Hover>> + Send + '_ {
        let uri = uri.to_string();
        async move { self.with_root(&uri, |root| self.hover_provider.hover(&root, range).map(|h| Hover { contents: h.contents, range: h.range })).await.flatten() }
    }
}
