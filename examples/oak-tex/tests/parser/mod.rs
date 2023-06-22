use oak_core::errors::OakError;
use oak_testing::parsing::ParserTester;
use oak_tex::{language::TexLanguage, parser::TexParser};
use std::{path::Path, time::Duration};

#[test]
fn test_tex_parser() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/parser");
    let language = TexLanguage::default();
    let parser = TexParser::new(language);
    let test_runner = ParserTester::new(tests).with_extension("tex").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&parser)
}
