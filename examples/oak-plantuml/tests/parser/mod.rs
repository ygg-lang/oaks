use oak_core::{ParseSession, parser::Parser};
use oak_plantuml::{language::PlantUmlLanguage, parser::PlantUmlParser};

#[test]
fn test_basic_parsing() {
    let language = PlantUmlLanguage::default();
    let parser = PlantUmlParser::new(&language);

    let content = "@startuml\nclass Test\n@interface TestInterface\n@enduml";
    let mut session = ParseSession::new(0);
    let parse_result = parser.parse(content, &[], &mut session);

    assert!(parse_result.result.is_ok());
    let root = parse_result.result.unwrap();
    assert!(root.children().len() > 0);
}

#[test]
fn test_comment_parsing() {
    let language = PlantUmlLanguage::default();
    let parser = PlantUmlParser::new(&language);

    let content = "@startuml\n// This is a comment\nclass Test\n@enduml";
    let mut session = ParseSession::new(0);
    let parse_result = parser.parse(content, &[], &mut session);

    assert!(parse_result.result.is_ok());
    let root = parse_result.result.unwrap();
    assert!(root.children().len() > 0);
}

#[test]
fn test_complex_parsing() {
    let language = PlantUmlLanguage::default();
    let parser = PlantUmlParser::new(&language);

    let content = "@startuml\nclass Test {\n  \"This is a label\"\n}\n@interface TestInterface\nTest --> TestInterface\n@enduml";
    let mut session = ParseSession::new(0);
    let parse_result = parser.parse(content, &[], &mut session);

    assert!(parse_result.result.is_ok());
    let root = parse_result.result.unwrap();
    assert!(root.children().len() > 0);
}

#[test]
fn test_empty_document() {
    let language = PlantUmlLanguage::default();
    let parser = PlantUmlParser::new(&language);

    let content = "";
    let mut session = ParseSession::new(0);
    let parse_result = parser.parse(content, &[], &mut session);

    assert!(parse_result.result.is_ok());
    let root = parse_result.result.unwrap();
    assert!(root.children().len() <= 1);
}
