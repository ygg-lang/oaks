use crate::{HandlebarsLanguage, kind::HandlebarsSyntaxKind};
use core::range::Range;
use futures::Future;
use oak_core::tree::RedNode;
use oak_hover::{Hover, HoverProvider};
use oak_lsp::service::LanguageService;
use oak_vfs::Vfs;

/// Hover provider implementation for Handlebars.
pub struct HandlebarsHoverProvider;

impl HoverProvider<HandlebarsLanguage> for HandlebarsHoverProvider {
    fn hover(&self, node: &RedNode<HandlebarsLanguage>, _range: Range<usize>) -> Option<Hover> {
        let kind = node.green.kind;

        // Provide context-aware hover information
        let contents = match kind {
            HandlebarsSyntaxKind::Mustache => "### Handlebars Mustache\nA basic template expression.",
            HandlebarsSyntaxKind::Block => "### Handlebars Block\nA block helper expression.",
            HandlebarsSyntaxKind::Partial => "### Handlebars Partial\nIncludes another template.",
            HandlebarsSyntaxKind::CommentNode => "### Handlebars Comment\nA template comment.",
            _ => return None,
        };

        Some(Hover { contents: contents.to_string(), range: Some(node.span()) })
    }
}

/// Language service implementation for Handlebars.
pub struct HandlebarsLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: HandlebarsHoverProvider,
}

impl<V: Vfs> HandlebarsLanguageService<V> {
    /// Creates a new `HandlebarsLanguageService`.
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: HandlebarsHoverProvider }
    }
}

impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for HandlebarsLanguageService<V> {
    type Lang = HandlebarsLanguage;
    type Vfs = V;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }

    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }

    fn get_root(&self, uri: &str) -> impl Future<Output = Option<RedNode<'_, HandlebarsLanguage>>> + Send + '_ {
        let source = self.vfs().get_source(uri);
        async move {
            let source = source?;
            let language = HandlebarsLanguage::default();
            let parser = crate::parser::HandlebarsParser::new(&language);
            let lexer = crate::lexer::HandlebarsLexer::new(&language);

            let mut cache = oak_core::parser::session::ParseSession::<HandlebarsLanguage>::default();
            let _output = oak_core::parser::parse(&parser, &lexer, &source, &[], &mut cache);
            None
        }
    }

    fn hover(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Option<oak_lsp::Hover>> + Send + '_ {
        let uri = uri.to_string();
        async move { self.with_root(&uri, |root| self.hover_provider.hover(&root, range).map(|h| oak_lsp::Hover { contents: h.contents, range: h.range })).await.flatten() }
    }
}
