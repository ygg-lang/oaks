use oak_core::parser::{ParseSession, Parser};
use oak_ocaml::{language::OCamlLanguage, parser::OCamlParser};

#[test]
fn test_ocaml_parser() {
    let lang = OCamlLanguage::new();
    let parser = OCamlParser::new(&lang);
    let source = "let x = 1 + 2\nlet f a b = a + b";
    let mut session = ParseSession::<OCamlLanguage>::new(16);
    let output = parser.parse(source, &[], &mut session);
    assert!(output.is_ok());
    let green = output.into_result().unwrap();
    assert!(green.children.len() > 0);
}
