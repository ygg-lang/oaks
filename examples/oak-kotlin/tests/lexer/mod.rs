use oak_core::{Lexer, NoLexerCache, SourceText};
use oak_kotlin::{KotlinLanguage, KotlinLexer};

#[test]
fn test_kotlin_lexer() {
    let source = SourceText::new("data class Person(val name: String, val age: Int)");
    let config = KotlinLanguage::new();
    let lexer = KotlinLexer::new(&config);
    let result = lexer.lex(&source, &[], &mut NoLexerCache);
    assert!(result.result.is_ok());
}
