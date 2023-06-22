use crate::{JsonLanguage, kind::JsonSyntaxKind};
use core::range::Range;
use futures::Future;
use oak_core::tree::RedNode;
use oak_hover::{Hover, HoverProvider};
use oak_lsp::service::LanguageService;
use oak_vfs::Vfs;

/// Hover provider implementation for JSON.
pub struct JsonHoverProvider;

impl HoverProvider<JsonLanguage> for JsonHoverProvider {
    fn hover(&self, node: &RedNode<JsonLanguage>, _range: Range<usize>) -> Option<Hover> {
        let kind = node.green.kind;

        // Provide context-aware hover information
        let contents = match kind {
            JsonSyntaxKind::Object => "### JSON Object\nA collection of key-value pairs.",
            JsonSyntaxKind::Array => "### JSON Array\nAn ordered list of values.",
            JsonSyntaxKind::ObjectEntry => "### JSON Property\nA key-value pair in an object.",
            JsonSyntaxKind::StringLiteral => "### JSON String\nA sequence of Unicode characters.",
            JsonSyntaxKind::NumberLiteral => "### JSON Number\nA numeric value.",
            JsonSyntaxKind::BooleanLiteral => "### JSON Boolean\nA true or false value.",
            JsonSyntaxKind::NullLiteral => "### JSON Null\nRepresents the intentional absence of any value.",
            _ => return None,
        };

        Some(Hover { contents: contents.to_string(), range: Some(node.span()) })
    }
}

/// Language service implementation for JSON.
pub struct JsonLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: JsonHoverProvider,
}

impl<V: Vfs> JsonLanguageService<V> {
    /// Creates a new `JsonLanguageService`.
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: JsonHoverProvider }
    }
}

impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for JsonLanguageService<V> {
    type Lang = JsonLanguage;
    type Vfs = V;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }

    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }

    fn get_root(&self, uri: &str) -> impl Future<Output = Option<RedNode<'_, JsonLanguage>>> + Send + '_ {
        let source = self.vfs().get_source(uri);
        async move {
            let source = source?;
            let language = JsonLanguage::default();
            let parser = crate::parser::JsonParser::new(&language);
            let lexer = crate::lexer::JsonLexer::new(&language);

            let mut cache = oak_core::parser::session::ParseSession::<JsonLanguage>::default();
            let _output = oak_core::parser::parse(&parser, &lexer, &source, &[], &mut cache);
            // In a real implementation, you would convert GreenNode to RedNode properly
            None
        }
    }

    fn hover(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Option<oak_lsp::Hover>> + Send + '_ {
        let uri = uri.to_string();
        async move { self.with_root(&uri, |root| self.hover_provider.hover(&root, range).map(|h| oak_lsp::Hover { contents: h.contents, range: h.range })).await.flatten() }
    }
}
