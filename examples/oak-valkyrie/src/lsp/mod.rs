use crate::{kind::ValkyrieSyntaxKind, language::ValkyrieLanguage};
use core::range::Range;
use oak_core::{Lexer, Parser, tree::RedNode};
use oak_hover::{Hover as ProviderHover, HoverProvider};
use oak_lsp::{service::LanguageService, types::Hover as LspHover};
use oak_vfs::Vfs;

/// Hover provider for Valkyrie.
pub struct ValkyrieHoverProvider;

impl HoverProvider<ValkyrieLanguage> for ValkyrieHoverProvider {
    fn hover(&self, node: &RedNode<ValkyrieLanguage>, _range: Range<usize>) -> Option<ProviderHover> {
        let kind = node.green.kind;

        let contents = match kind {
            ValkyrieSyntaxKind::Function => "### Valkyrie Function\nDefines a callable function.",
            ValkyrieSyntaxKind::Namespace => "### Valkyrie Namespace\nDefines a scope for items.",
            ValkyrieSyntaxKind::Micro => "### Valkyrie Micro\nDefines a micro-service or component.",
            ValkyrieSyntaxKind::LetStatement => "### Let Statement\nDeclares a local variable.",
            ValkyrieSyntaxKind::Identifier => "### Identifier\nA name referring to a value or item.",
            _ => return None,
        };

        Some(ProviderHover { contents: contents.to_string(), range: Some(node.span()) })
    }
}

/// Language service implementation for Valkyrie.
pub struct ValkyrieLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: ValkyrieHoverProvider,
}

impl<V: Vfs> ValkyrieLanguageService<V> {
    /// Creates a new Valkyrie language service.
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: ValkyrieHoverProvider }
    }
}

impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for ValkyrieLanguageService<V> {
    type Lang = ValkyrieLanguage;
    type Vfs = V;

    fn vfs(&self) -> &V {
        &self.vfs
    }

    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }

    fn get_root(&self, uri: &str) -> impl futures::Future<Output = Option<RedNode<'_, ValkyrieLanguage>>> + Send + '_ {
        let source = self.get_source(uri);
        async move {
            let source = source?;
            let language = ValkyrieLanguage::default();
            let parser = crate::parser::ValkyrieParser::new(&language);
            let lexer = crate::lexer::ValkyrieLexer::new(&language);

            // 使用 Box::leak 或类似的方案来延长 cache 的生命周期是不安全的，
            // 更好的做法是将 cache 和 green 节点一起管理，或者重新设计 RedNode。
            // 在当前的 oak-core 设计中，RedNode 必须引用 GreenNode，而 GreenNode
            // 通常由 ParseHeap 分配在 Arena 中。

            let mut cache = Box::new(oak_core::parser::session::ParseSession::<ValkyrieLanguage>::default());
            let cache_ptr: *mut oak_core::parser::session::ParseSession<ValkyrieLanguage> = &mut *cache;

            lexer.lex(source.as_ref(), &[], unsafe { &mut *cache_ptr });
            let output = parser.parse(source.as_ref(), &[], unsafe { &mut *cache_ptr });
            let green = output.result.ok()?;

            // 这里的 green 实际上是从 cache.arena 分配的。
            // 在 LSP 中，正确的做法是将解析结果存入 WorkspaceManager。

            let _leaked_cache = Box::leak(cache);
            // 安全地获取 'static 生命周期的 green 节点，因为它现在属于 leaked_cache
            let green_static: &'static oak_core::GreenNode<ValkyrieLanguage> = unsafe { std::mem::transmute(green) };

            Some(RedNode::new(green_static, 0))
        }
    }

    fn hover(&self, uri: &str, range: Range<usize>) -> impl futures::Future<Output = Option<LspHover>> + Send + '_ {
        let uri = uri.to_string();
        async move {
            self.with_root(&uri, |root| {
                use oak_core::tree::RedTree;
                let node = match root.child_at_offset(range.start) {
                    Some(RedTree::Node(n)) => n,
                    _ => root,
                };
                self.hover_provider.hover(&node, range).map(|h| LspHover { contents: h.contents, range: h.range })
            })
            .await
            .flatten()
        }
    }
}
