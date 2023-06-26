use oak_core::{ParseSession, Parser, SourceText};
use oak_vue::{VueLanguage, VueParser};

#[test]
fn test_simple_vue() {
    let source = SourceText::new("<template><div>{{ msg }}</div></template>");
    let language = VueLanguage::default();
    let parser = VueParser::new(&language);
    let mut session = ParseSession::default();

    let result = parser.parse(&source, &[], &mut session);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors");

    println!("Parse successful!");
}
