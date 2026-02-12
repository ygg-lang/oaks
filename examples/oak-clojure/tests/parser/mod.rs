use oak_clojure::{language::ClojureLanguage, parser::ClojureParser};
use oak_core::parser::{ParseSession, Parser};

#[test]
fn test_clojure_parser() {
    let lang = ClojureLanguage::new();
    let parser = ClojureParser::new(&lang);
    let source = "(ns main)\n(defn hello [] (println \"Hello\"))\n(hello)";
    let mut session = ParseSession::<ClojureLanguage>::new(16);
    let output = parser.parse(source, &[], &mut session);
    assert!(output.is_ok());
    let green = output.into_result().unwrap();
    assert!(green.children.len() > 0);
}
