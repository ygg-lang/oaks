use oak_core::helpers::LexerTester;
use oak_lua::{LuaLanguage, LuaLexer};
use std::path::Path;

#[test]
fn test_lua_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let language = LuaLanguage::default();
    let lexer = LuaLexer::new(&language);
    let test_runner = LexerTester::new(tests).with_extension("lua");
    match test_runner.run_tests::<LuaLanguage, _>(lexer) {
        Ok(()) => println!("Lua lexer tests passed!"),
        Err(e) => panic!("Lua lexer tests failed: {}", e),
    }
}
