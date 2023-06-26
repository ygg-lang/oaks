use oak_core::{ParseSession, parser::Parser};
use oak_structurizr::{language::StructurizrLanguage, parser::StructurizrParser};

#[test]
fn test_basic_parsing() {
    let language = StructurizrLanguage::default();
    let parser = StructurizrParser::new(&language);

    let content = "workspace \"My Workspace\" {\n    model {\n        person \"User\"\n        softwareSystem \"Software System\" {\n            container \"Web App\" {\n                component \"Controller\"\n            }\n        }\n    }\n}";
    let mut session = ParseSession::new(0);
    let parse_result = parser.parse(content, &[], &mut session);

    assert!(parse_result.result.is_ok());
    let root = parse_result.result.unwrap();
    assert!(root.children().len() > 0);
}

#[test]
fn test_workspace_parsing() {
    let language = StructurizrLanguage::default();
    let parser = StructurizrParser::new(&language);

    let content = "workspace \"My Workspace\" {\n    // Workspace content\n}";
    let mut session = ParseSession::new(0);
    let parse_result = parser.parse(content, &[], &mut session);

    assert!(parse_result.result.is_ok());
    let root = parse_result.result.unwrap();
    assert!(root.children().len() > 0);
}

#[test]
fn test_model_parsing() {
    let language = StructurizrLanguage::default();
    let parser = StructurizrParser::new(&language);

    let content = "model {\n    person \"User\"\n    softwareSystem \"Software System\"\n}";
    let mut session = ParseSession::new(0);
    let parse_result = parser.parse(content, &[], &mut session);

    assert!(parse_result.result.is_ok());
    let root = parse_result.result.unwrap();
    assert!(root.children().len() > 0);
}

#[test]
fn test_person_parsing() {
    let language = StructurizrLanguage::default();
    let parser = StructurizrParser::new(&language);

    let content = "person \"User\"";
    let mut session = ParseSession::new(0);
    let parse_result = parser.parse(content, &[], &mut session);

    assert!(parse_result.result.is_ok());
    let root = parse_result.result.unwrap();
    assert!(root.children().len() > 0);
}

#[test]
fn test_software_system_parsing() {
    let language = StructurizrLanguage::default();
    let parser = StructurizrParser::new(&language);

    let content = "softwareSystem \"Software System\" {\n    container \"Web App\"\n}";
    let mut session = ParseSession::new(0);
    let parse_result = parser.parse(content, &[], &mut session);

    assert!(parse_result.result.is_ok());
    let root = parse_result.result.unwrap();
    assert!(root.children().len() > 0);
}

#[test]
fn test_container_parsing() {
    let language = StructurizrLanguage::default();
    let parser = StructurizrParser::new(&language);

    let content = "container \"Web App\" {\n    component \"Controller\"\n}";
    let mut session = ParseSession::new(0);
    let parse_result = parser.parse(content, &[], &mut session);

    assert!(parse_result.result.is_ok());
    let root = parse_result.result.unwrap();
    assert!(root.children().len() > 0);
}

#[test]
fn test_component_parsing() {
    let language = StructurizrLanguage::default();
    let parser = StructurizrParser::new(&language);

    let content = "component \"Controller\"";
    let mut session = ParseSession::new(0);
    let parse_result = parser.parse(content, &[], &mut session);

    assert!(parse_result.result.is_ok());
    let root = parse_result.result.unwrap();
    assert!(root.children().len() > 0);
}
