use oak_core::helpers::ParserTester;
use oak_handlebars::{HandlebarsLanguage, HandlebarsParser};
use std::{path::Path, time::Duration};

#[test]
fn test_handlebars_parser() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(HandlebarsLanguage::default()));
    let parser = HandlebarsParser::new(language);
    let test_runner = ParserTester::new(here.join("tests/parser")).with_extension("hbs").with_timeout(Duration::from_secs(5));

    match test_runner.run_tests::<HandlebarsLanguage, _>(&parser) {
        Ok(()) => println!("Handlebars parser tests passed!"),
        Err(e) => panic!("Handlebars parser tests failed: {}", e),
    }
}
