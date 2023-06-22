use crate::{RustLanguage, parser::RustElementType};
use core::range::Range;
use futures::Future;
use oak_core::tree::RedNode;
use oak_hover::{Hover, HoverProvider};
use oak_lsp::service::LanguageService;
use oak_vfs::Vfs;

/// Hover provider implementation for Rust.
pub struct RustHoverProvider;

impl HoverProvider<RustLanguage> for RustHoverProvider {
    fn hover(&self, node: &RedNode<RustLanguage>, _range: Range<usize>) -> Option<Hover> {
        let kind = node.green.kind;

        // Provide context-aware hover information
        let contents = match kind {
            RustElementType::Function => "### Rust Function\nDefines a callable block of code.",
            RustElementType::StructItem => "### Rust Struct\nDefines a custom data type.",
            RustElementType::ModuleItem => "### Rust Module\nOrganizes code into namespaces.",
            RustElementType::Trait => "### Rust Trait\nDefines a shared behavior.",
            _ => return None,
        };

        Some(Hover { contents: contents.to_string(), range: Some(node.span()) })
    }
}

/// Language service implementation for Rust.
pub struct RustLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: RustHoverProvider,
}

impl<V: Vfs> RustLanguageService<V> {
    /// Creates a new `RustLanguageService`.
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: RustHoverProvider }
    }
}

impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for RustLanguageService<V> {
    type Lang = RustLanguage;
    type Vfs = V;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }

    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }

    fn get_root(&self, uri: &str) -> impl Future<Output = Option<RedNode<'_, RustLanguage>>> + Send + '_ {
        let source = self.vfs().get_source(uri);
        async move {
            let source = source?;
            let language = RustLanguage::default();
            let parser = crate::parser::RustParser::new(&language);
            let lexer = crate::lexer::RustLexer::new(&language);

            // In a real implementation, you would use a cache
            let mut cache = oak_core::parser::session::ParseSession::<RustLanguage>::default();
            let _output = oak_core::parser::parse(&parser, &lexer, &source, &[], &mut cache);
            // Convert GreenNode to RedNode
            // NOTE: This is a temporary hack because RedNode requires a reference to GreenNode
            // which is owned by output. In a real implementation, the cache or workspace
            // would own the GreenNode.
            None
        }
    }

    fn hover(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Option<oak_lsp::Hover>> + Send + '_ {
        let uri = uri.to_string();
        async move {
            self.with_root(&uri, |root| {
                // In a real implementation, you would find the specific node at offset
                // For this example, we just check the root or simple children
                self.hover_provider.hover(&root, range).map(|h| oak_lsp::Hover { contents: h.contents, range: h.range })
            })
            .await
            .flatten()
        }
    }
}
