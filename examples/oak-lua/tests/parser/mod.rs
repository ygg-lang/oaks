use oak_core::errors::OakError;
use oak_lua::{LuaLanguage, LuaParser};
use oak_testing::parsing::ParserTester;
use std::{path::Path, time::Duration};

#[test]
fn test_lua_parser() -> Result<(), OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/parser");
    let language = Box::leak(Box::new(LuaLanguage::default()));
    let parser = LuaParser::new(language);
    let test_runner = ParserTester::new(tests).with_extension("lua").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&parser)?;
    Ok(())
}
