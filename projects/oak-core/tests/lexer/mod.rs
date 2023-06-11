use oak_core::{Language, Lexer, SourceText, SyntaxKind, Token, helpers::LexerTester, lexer::LexOutput};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[test]
fn test_lexer_framework() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = SimpleLexer;
    let test_runner = LexerTester::new(tests).with_extension("txt");
    match test_runner.run_tests::<SimpleLanguage, _>(lexer) {
        Ok(()) => println!("All lexer tests passed!"),
        Err(e) => panic!("Lexer tests failed: {}", e),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SimpleToken {
    Identifier,
    Number,
    Plus,
    Minus,
    Star,
    Slash,
    Whitespace,
    Error,
}

impl SyntaxKind for SimpleToken {
    fn is_trivia(&self) -> bool {
        matches!(self, SimpleToken::Whitespace)
    }

    fn is_comment(&self) -> bool {
        false
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, SimpleToken::Whitespace)
    }

    fn is_token_type(&self) -> bool {
        true
    }

    fn is_element_type(&self) -> bool {
        false
    }
}

pub struct SimpleLanguage;

impl Language for SimpleLanguage {
    type SyntaxKind = SimpleToken;
}

pub struct SimpleLexer;

impl Lexer<SimpleLanguage> for SimpleLexer {
    fn lex(&self, _: &SourceText) -> LexOutput<SimpleToken> {
        LexOutput { result: Ok(vec![Token { kind: SimpleToken::Error, span: Default::default() }]), diagnostics: vec![] }
    }
}
