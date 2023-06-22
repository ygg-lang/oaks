#![feature(new_range_api)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

pub use oak_core::*;
pub use oak_folding::{FoldingProvider, FoldingRange};
pub use oak_hover::{Hover, HoverProvider};
pub use oak_lsp::{FoldingRangeKind, LanguageService, Location, LspRange, Position, Range};
pub use oak_navigation::{DefinitionProvider, ReferencesProvider};
pub use oak_semantic_tokens::{SemanticToken, SemanticTokensProvider};
pub use oak_structure::{StructureItem, StructureProvider};
pub use oak_symbols::{SymbolInformation, SymbolProvider};
pub use oak_vfs::{FileMetadata, FileType, MemoryVfs, Vfs};

mod languages;
