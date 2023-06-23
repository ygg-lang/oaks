#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![warn(missing_docs)]
//! Wolfram support for the Oak language framework.

pub mod ast;
mod builder;
pub mod language;
pub mod lexer;
#[cfg(any(feature = "lsp", feature = "oak-highlight", feature = "oak-pretty-print"))]
pub mod lsp;
pub mod parser;

pub use crate::{builder::WolframBuilder, language::WolframLanguage, lexer::WolframLexer, parser::WolframParser};
pub use lexer::token_type::WolframTokenType;
pub use oak_core::{ElementType, TokenType};
pub use parser::element_type::WolframElementType;

#[cfg(test)]
mod tests {
    use super::*;
    use oak_core::{Builder, SourceText, parser::ParseSession};

    #[test]
    fn test_functional_parsing() {
        let language = WolframLanguage::default();
        let builder = WolframBuilder::new(&language);
        let mut session = ParseSession::<WolframLanguage>::default();

        let code = "f @ x + g /@ list // h";
        let source = SourceText::new(code);
        let output = builder.build(&source, &[], &mut session);

        assert!(output.result.is_ok());
        // 可以在这里进一步检查树的结构，但至少解析成功了
    }

    #[test]
    fn test_pure_function() {
        let language = WolframLanguage::default();
        let builder = WolframBuilder::new(&language);
        let mut session = ParseSession::<WolframLanguage>::default();

        let code = "# + 1 &";
        let source = SourceText::new(code);
        let output = builder.build(&source, &[], &mut session);

        assert!(output.result.is_ok());
    }

    #[test]
    fn test_apply_level_and_map_all() {
        let language = WolframLanguage::default();
        let builder = WolframBuilder::new(&language);
        let mut session = ParseSession::<WolframLanguage>::default();

        let code = "f @@@ expr + g //@ list";
        let source = SourceText::new(code);
        let output = builder.build(&source, &[], &mut session);

        assert!(output.result.is_ok());
    }

    #[test]
    fn test_nested_calls() {
        let language = WolframLanguage::default();
        let builder = WolframBuilder::new(&language);
        let mut session = ParseSession::<WolframLanguage>::default();

        let code = "f[x][y][z]";
        let source = SourceText::new(code);
        let output = builder.build(&source, &[], &mut session);

        assert!(output.result.is_ok());
    }
}
