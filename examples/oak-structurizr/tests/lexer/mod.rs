use oak_structurizr::{
    language::StructurizrLanguage,
    lexer::{StructurizrLexer, token_type::StructurizrTokenType},
};

#[test]
fn test_basic_lexing() {
    let language = StructurizrLanguage::default();
    let lexer = StructurizrLexer::new(&language);

    let content = "workspace \"My Workspace\" {\n    model {\n        person \"User\"\n        softwareSystem \"Software System\" {\n            container \"Web App\" {\n                component \"Controller\"\n            }\n        }\n    }\n}";
    let lex_output = lexer.lex_internal(content);

    assert!(lex_output.result.is_ok());
    let tokens = lex_output.result.unwrap();
    assert!(!tokens.is_empty());
}

#[test]
fn test_keywords() {
    let language = StructurizrLanguage::default();
    let lexer = StructurizrLexer::new(&language);

    let content = "workspace model person softwareSystem container component";
    let lex_output = lexer.lex_internal(content);

    assert!(lex_output.result.is_ok());
    let tokens = lex_output.result.unwrap();
    assert!(!tokens.is_empty());

    // 检查是否有所有关键字标记
    let has_workspace = tokens.iter().any(|token| token.kind == StructurizrTokenType::Workspace);
    let has_model = tokens.iter().any(|token| token.kind == StructurizrTokenType::Model);
    let has_person = tokens.iter().any(|token| token.kind == StructurizrTokenType::Person);
    let has_software_system = tokens.iter().any(|token| token.kind == StructurizrTokenType::SoftwareSystem);
    let has_container = tokens.iter().any(|token| token.kind == StructurizrTokenType::Container);
    let has_component = tokens.iter().any(|token| token.kind == StructurizrTokenType::Component);

    assert!(has_workspace);
    assert!(has_model);
    assert!(has_person);
    assert!(has_software_system);
    assert!(has_container);
    assert!(has_component);
}

#[test]
fn test_comments() {
    let language = StructurizrLanguage::default();
    let lexer = StructurizrLexer::new(&language);

    let content = "// This is a comment\nworkspace \"My Workspace\"";
    let lex_output = lexer.lex_internal(content);

    assert!(lex_output.result.is_ok());
    let tokens = lex_output.result.unwrap();
    assert!(!tokens.is_empty());

    // 检查是否有注释标记
    let has_comment = tokens.iter().any(|token| token.kind == StructurizrTokenType::Comment);
    assert!(has_comment);
}

#[test]
fn test_strings() {
    let language = StructurizrLanguage::default();
    let lexer = StructurizrLexer::new(&language);

    let content = "workspace \"My Workspace\"";
    let lex_output = lexer.lex_internal(content);

    assert!(lex_output.result.is_ok());
    let tokens = lex_output.result.unwrap();
    assert!(!tokens.is_empty());

    // 检查是否有标签标记
    let has_label = tokens.iter().any(|token| token.kind == StructurizrTokenType::Label);
    assert!(has_label);
}

#[test]
fn test_braces() {
    let language = StructurizrLanguage::default();
    let lexer = StructurizrLexer::new(&language);

    let content = "workspace { }";
    let lex_output = lexer.lex_internal(content);

    assert!(lex_output.result.is_ok());
    let tokens = lex_output.result.unwrap();
    assert!(!tokens.is_empty());

    // 检查是否有左右大括号标记
    let has_left_brace = tokens.iter().any(|token| token.kind == StructurizrTokenType::LeftBrace);
    let has_right_brace = tokens.iter().any(|token| token.kind == StructurizrTokenType::RightBrace);

    assert!(has_left_brace);
    assert!(has_right_brace);
}
