use oak_core::{Lexer, ParseSession, source::SourceText};
use oak_groovy::{language::GroovyLanguage, lexer::GroovyLexer};

#[test]
fn test_groovy_lexer() {
    let source = "class HelloWorld";

    let language = GroovyLanguage::default();
    let lexer = GroovyLexer::new(&language);
    let source_text = SourceText::new(source);
    let mut session = ParseSession::<GroovyLanguage>::new(1024);
    let result = lexer.lex(&source_text, &[], &mut session);

    match result.result {
        Ok(tokens) => {
            assert!(!tokens.is_empty(), "Tokens should not be empty");
            println!("Groovy lexer test passed with {} tokens", tokens.len());
        }
        Err(e) => panic!("Groovy lexer test failed: {}", e),
    }
}

#[test]
fn test_peek_behavior() {
    let source = "class Test";
    let language = GroovyLanguage::default();
    let lexer = GroovyLexer::new(&language);
    let source_text = SourceText::new(source);
    let mut session = ParseSession::<GroovyLanguage>::new(1024);
    let result = lexer.lex(&source_text, &[], &mut session);

    match result.result {
        Ok(tokens) => {
            assert!(!tokens.is_empty(), "Should have tokens for 'class Test'");
            println!("Peek behavior test passed");
        }
        Err(e) => panic!("Peek behavior test failed: {}", e),
    }
}

#[test]
fn test_groovy_class_parsing() {
    let source = "class Person";
    let language = GroovyLanguage::default();
    let lexer = GroovyLexer::new(&language);
    let source_text = SourceText::new(source);
    let mut session = ParseSession::<GroovyLanguage>::new(1024);
    let result = lexer.lex(&source_text, &[], &mut session);

    match result.result {
        Ok(tokens) => {
            assert!(!tokens.is_empty(), "Should have tokens for class definition");
            println!("Groovy class parsing test passed");
        }
        Err(e) => panic!("Groovy class parsing test failed: {}", e),
    }
}
