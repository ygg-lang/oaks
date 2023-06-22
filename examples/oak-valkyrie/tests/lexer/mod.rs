use oak_core::helpers::LexerTester;
use oak_valkyrie::{ValkyrieLanguage, ValkyrieLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_valkyrie_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(ValkyrieLanguage::default()));
    let lexer = ValkyrieLexer::new(language);
    // use `valkyrie` extension for Valkyrie source files
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("valkyrie").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<ValkyrieLanguage, _>(&lexer) {
        Ok(()) => println!("Valkyrie lexer tests passed!"),
        Err(e) => panic!("Valkyrie lexer tests failed: {}", e),
    }
}

#[test]
fn test_valkyrie_keywords() {
    use oak_core::SourceText;
    use oak_valkyrie::{ValkyrieLanguage, ValkyrieLexer, ValkyrieSyntaxKind, lexer::ValkyrieKeywords};

    let language = ValkyrieLanguage::default();
    let lexer = ValkyrieLexer::new(&language);

    // Test Valkyrie keywords
    let source = SourceText::new("namespace micro fn let if else while for return break continue true false null");
    let tokens: Vec<_> = lexer.tokenize(&source).collect();

    println!("Valkyrie keywords test - {} tokens generated", tokens.len());

    // Check that we have the expected keywords
    let _keyword_kinds = [
        ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Namespace),
        ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Micro),
        ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Fn),
        ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Let),
        ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::If),
        ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Else),
        ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::While),
        ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::For),
        ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Return),
        ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Break),
        ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Continue),
        ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::True),
        ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::False),
        ValkyrieSyntaxKind::Keyword(ValkyrieKeywords::Null),
    ];

    assert!(!tokens.is_empty(), "Should generate tokens for keywords");
}

#[test]
fn test_valkyrie_identifiers() {
    use oak_core::SourceText;
    use oak_valkyrie::{ValkyrieLanguage, ValkyrieLexer};

    let language = ValkyrieLanguage::default();
    let lexer = ValkyrieLexer::new(&language);

    // Test identifiers
    let source = SourceText::new("myVariable myFunction MyClass _private");
    let tokens: Vec<_> = lexer.tokenize(&source).collect();

    println!("Valkyrie identifiers test - {} tokens generated", tokens.len());
    assert!(!tokens.is_empty(), "Should generate tokens for identifiers");
}

#[test]
fn test_valkyrie_literals() {
    use oak_core::SourceText;
    use oak_valkyrie::{ValkyrieLanguage, ValkyrieLexer};

    let language = ValkyrieLanguage::default();
    let lexer = ValkyrieLexer::new(&language);

    // Test literals
    let source = SourceText::new(r#"42 3.14 "hello world" true false"#);
    let tokens: Vec<_> = lexer.tokenize(&source).collect();

    println!("Valkyrie literals test - {} tokens generated", tokens.len());
    assert!(!tokens.is_empty(), "Should generate tokens for literals");
}

#[test]
fn test_valkyrie_operators() {
    use oak_core::SourceText;
    use oak_valkyrie::{ValkyrieLanguage, ValkyrieLexer};

    let language = ValkyrieLanguage::default();
    let lexer = ValkyrieLexer::new(&language);

    // Test operators
    let source = SourceText::new("+ - * / = == != < > <= >= && || ! & | ^ << >>");
    let tokens: Vec<_> = lexer.tokenize(&source).collect();

    println!("Valkyrie operators test - {} tokens generated", tokens.len());
    assert!(!tokens.is_empty(), "Should generate tokens for operators");
}

#[test]
fn test_valkyrie_punctuation() {
    use oak_core::SourceText;
    use oak_valkyrie::{ValkyrieLanguage, ValkyrieLexer};

    let language = ValkyrieLanguage::default();
    let lexer = ValkyrieLexer::new(&language);

    // Test punctuation
    let source = SourceText::new("{ } ( ) [ ] ; , . :");
    let tokens: Vec<_> = lexer.tokenize(&source).collect();

    println!("Valkyrie punctuation test - {} tokens generated", tokens.len());
    assert!(!tokens.is_empty(), "Should generate tokens for punctuation");
}
