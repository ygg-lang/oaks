use oak_handlebars::{HandlebarsLanguage, HandlebarsParser};
use oak_testing::parsing::ParserTester;
use std::{path::Path, time::Duration};

#[test]
fn test_handlebars_parser() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = HandlebarsLanguage::default();
    let parser = HandlebarsParser::new(&language);
    let test_runner = ParserTester::new(here.join("tests/files")).with_extension("hbs").with_timeout(Duration::from_secs(5));

    test_runner.run_tests(&parser)
}
