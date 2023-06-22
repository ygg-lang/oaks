use crate::{CoqLanguage, kind::CoqSyntaxKind};
use core::range::Range;
use futures::Future;
use oak_core::tree::RedNode;
use oak_hover::{Hover, HoverProvider};
use oak_lsp::service::LanguageService;
use oak_vfs::Vfs;

/// Hover provider implementation for Coq.
pub struct CoqHoverProvider;

impl HoverProvider<CoqLanguage> for CoqHoverProvider {
    fn hover(&self, node: &RedNode<CoqLanguage>, _range: Range<usize>) -> Option<Hover> {
        let kind = node.green.kind;

        // Provide context-aware hover information
        let contents = match kind {
            CoqSyntaxKind::Declaration => "### Coq Declaration\nDefines a new term, theorem, or inductive type.",
            CoqSyntaxKind::Statement => "### Coq Statement\nA vernacular command or tactic.",
            CoqSyntaxKind::Expression => "### Coq Expression\nA Coq term or type.",
            _ => return None,
        };

        Some(Hover { contents: contents.to_string(), range: Some(node.span()) })
    }
}

/// Language service implementation for Coq.
pub struct CoqLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: CoqHoverProvider,
}

impl<V: Vfs> CoqLanguageService<V> {
    /// Creates a new `CoqLanguageService`.
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: CoqHoverProvider }
    }
}

impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for CoqLanguageService<V> {
    type Lang = CoqLanguage;
    type Vfs = V;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }

    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }

    fn get_root(&self, uri: &str) -> impl Future<Output = Option<RedNode<'_, CoqLanguage>>> + Send + '_ {
        let source = self.vfs().get_source(uri);
        async move {
            let source = source?;
            let language = CoqLanguage::default();
            let parser = crate::parser::CoqParser::new(&language);
            let lexer = crate::lexer::CoqLexer::new(&language);

            let mut cache = Box::new(oak_core::parser::session::ParseSession::<CoqLanguage>::default());
            let cache_ptr: *mut oak_core::parser::session::ParseSession<CoqLanguage> = &mut *cache;

            let parse_out = oak_core::parser::parse(&parser, &lexer, &source, &[], unsafe { &mut *cache_ptr });
            let green = parse_out.result.ok()?;

            let _leaked_cache = Box::leak(cache);
            let green_static: &'static oak_core::GreenNode<CoqLanguage> = unsafe { std::mem::transmute(green) };

            Some(RedNode::new(green_static, 0))
        }
    }

    fn hover(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Option<oak_lsp::Hover>> + Send + '_ {
        let uri = uri.to_string();
        async move { self.with_root(&uri, |root| self.hover_provider.hover(&root, range).map(|h| oak_lsp::Hover { contents: h.contents, range: h.range })).await.flatten() }
    }
}
