//! Go 语言 LSP 服务

use crate::language::GoLanguage;
use oak_lsp::LanguageService;

/// Go 语言服务
pub type GoLanguageService = dyn LanguageService<Lang = GoLanguage, Vfs = oak_vfs::MemoryVfs>;
