//! Oak Lean 语言支持
//!
//! Oak 解析器框架提Lean 语言的词法分析和语法支持
pub mod language;
pub mod lexer;
pub mod syntax;

pub use language::LeanLanguage;
pub use lexer::LeanLexer;
pub use syntax::LeanSyntaxKind;
