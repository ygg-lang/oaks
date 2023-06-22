use crate::{kind::ProtobufSyntaxKind, language::ProtobufLanguage};
use oak_core::{Range, tree::RedNode};
use oak_lsp::{service::LanguageService, types::Hover};
use oak_vfs::Vfs;
use std::future::Future;

/// Hover provider implementation for Protobuf.
pub struct ProtobufHoverProvider;

impl ProtobufHoverProvider {
    fn hover(&self, node: &RedNode<ProtobufLanguage>, _range: Range<usize>) -> Option<Hover> {
        let kind = node.green.kind;

        let contents = match kind {
            ProtobufSyntaxKind::MessageDef => "### Protobuf Message\nA message is a collection of fields. It's the primary way to define data structures in Protobuf.",
            ProtobufSyntaxKind::EnumDef => "### Protobuf Enum\nAn enum is a set of named integer constants.",
            ProtobufSyntaxKind::ServiceDef => "### Protobuf Service\nA service defines a set of RPC methods that can be called remotely.",
            _ => return None,
        };

        Some(Hover { contents: contents.to_string(), range: Some(node.span()) })
    }
}

/// Language service implementation for Protobuf.
pub struct ProtobufLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: ProtobufHoverProvider,
}

impl<V: Vfs> ProtobufLanguageService<V> {
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: ProtobufHoverProvider }
    }
}

impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for ProtobufLanguageService<V> {
    type Lang = ProtobufLanguage;
    type Vfs = V;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }

    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }

    fn get_root(&self, _uri: &str) -> impl Future<Output = Option<RedNode<'_, ProtobufLanguage>>> + Send + '_ {
        async move { None }
    }

    fn hover(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Option<Hover>> + Send + '_ {
        let uri = uri.to_string();
        async move { self.with_root(&uri, |root| self.hover_provider.hover(&root, range)).await.flatten() }
    }
}
