// LLVM IR language support for Oak
// This is a placeholder implementation

pub mod kind;
pub mod language;
pub mod lexer;

pub use crate::kind::LlvmKind;
pub use crate::language::LlvmLanguage;
pub use crate::lexer::LlvmLexer;