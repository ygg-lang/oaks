use oak_core::parser::{ParseSession, Parser};
use oak_haskell::{HaskellLanguage, HaskellParser};

#[test]
fn test_haskell_parser() {
    let lang = HaskellLanguage::new();
    let parser = HaskellParser::new(&lang);
    let source = "module Main where\nimport Data.List\nmain = print (1 + 2)";
    let mut session = ParseSession::<HaskellLanguage>::new(16);
    let output = parser.parse(source, &[], &mut session);
    assert!(output.is_ok());
    let green = output.into_result().unwrap();
    assert!(green.children.len() > 0);
}
