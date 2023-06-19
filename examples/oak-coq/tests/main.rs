use oak_coq::CoqLanguage;
use oak_core::{GreenBuilder, IncrementalCache, SourceText, lexer::Lexer};

#[test]
fn test_basic_syntax() {
    let source = SourceText::new("Definition x := 1.");
    let language = CoqLanguage::new();
    let lexer = language.lexer();
    let mut pool = GreenBuilder::<CoqLanguage>::new(100);
    let cache = IncrementalCache::new(&mut pool);

    let result = lexer.lex_incremental(&source, 0, cache);

    assert!(result.result.is_ok());
    let tokens = result.result.unwrap();
    assert!(!tokens.is_empty());
}

#[test]
fn test_keywords() {
    let source = SourceText::new("Theorem Proof Qed");
    let language = CoqLanguage::new();
    let lexer = language.lexer();
    let mut pool = GreenBuilder::<CoqLanguage>::new(100);
    let cache = IncrementalCache::new(&mut pool);

    let result = lexer.lex_incremental(&source, 0, cache);

    assert!(result.result.is_ok());
    let tokens = result.result.unwrap();
    assert!(tokens.len() >= 3);
}

#[test]
fn test_comments() {
    let source = SourceText::new("(* This is a comment *)");
    let language = CoqLanguage::new();
    let lexer = language.lexer();
    let mut pool = GreenBuilder::<CoqLanguage>::new(100);
    let cache = IncrementalCache::new(&mut pool);

    let result = lexer.lex_incremental(&source, 0, cache);

    assert!(result.result.is_ok());
}

#[test]
fn test_strings() {
    let source = SourceText::new("\"hello world\"");
    let language = CoqLanguage::new();
    let lexer = language.lexer();
    let mut pool = GreenBuilder::<CoqLanguage>::new(100);
    let cache = IncrementalCache::new(&mut pool);

    let result = lexer.lex_incremental(&source, 0, cache);

    assert!(result.result.is_ok());
}

#[test]
fn test_numbers() {
    let source = SourceText::new("123 456.789");
    let language = CoqLanguage::new();
    let lexer = language.lexer();
    let mut pool = GreenBuilder::<CoqLanguage>::new(100);
    let cache = IncrementalCache::new(&mut pool);

    let result = lexer.lex_incremental(&source, 0, cache);

    assert!(result.result.is_ok());
}
