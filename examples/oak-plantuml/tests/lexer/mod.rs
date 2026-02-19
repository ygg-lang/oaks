use oak_plantuml::{
    language::PlantUmlLanguage,
    lexer::{PlantUmlLexer, token_type::PlantUmlTokenType},
};

#[test]
fn test_basic_lexing() {
    let language = PlantUmlLanguage::default();
    let lexer = PlantUmlLexer::new(&language);

    let content = "@startuml\nclass Test\n@interface TestInterface\n@enduml";
    let lex_output = lexer.lex_internal(content);

    assert!(lex_output.result.is_ok());
    let tokens = lex_output.result.unwrap();
    assert!(!tokens.is_empty());
}

#[test]
fn test_comment_lexing() {
    let language = PlantUmlLanguage::default();
    let lexer = PlantUmlLexer::new(&language);

    let content = "@startuml\n// This is a comment\nclass Test\n@enduml";
    let lex_output = lexer.lex_internal(content);

    assert!(lex_output.result.is_ok());
    let tokens = lex_output.result.unwrap();
    assert!(!tokens.is_empty());
    // 检查是否包含注释标记
    let has_comment = tokens.iter().any(|t| t.kind == PlantUmlTokenType::Comment);
    assert!(has_comment);
}

#[test]
fn test_label_lexing() {
    let language = PlantUmlLanguage::default();
    let lexer = PlantUmlLexer::new(&language);

    let content = "@startuml\nclass Test {\n  \"This is a label\"\n}\n@enduml";
    let lex_output = lexer.lex_internal(content);

    assert!(lex_output.result.is_ok());
    let tokens = lex_output.result.unwrap();
    assert!(!tokens.is_empty());
    // 检查是否包含标签标记
    let has_label = tokens.iter().any(|t| t.kind == PlantUmlTokenType::Label);
    assert!(has_label);
}

#[test]
fn test_error_handling() {
    let language = PlantUmlLanguage::default();
    let lexer = PlantUmlLexer::new(&language);

    let content = "@startuml\nclass Test\n@enduml";
    let lex_output = lexer.lex_internal(content);

    assert!(lex_output.result.is_ok());
    let tokens = lex_output.result.unwrap();
    assert!(!tokens.is_empty());
}
