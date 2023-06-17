use oak_core::{helpers::run_parser_tests, SourceText};
use oak_rust::RustParser;
use oak_rust::language::RustLanguage;
use std::path::Path;

#[test]
fn test_rust_parser() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/parser");

    // 构造一个解析器实例；其 parse 方法会基于传入的 SourceText 构造新的内部解析器
    let dummy = SourceText::new("");
    let parser = RustParser::new(&dummy, &[]);

    match run_parser_tests::<RustLanguage, _>(tests, "txt", parser) {
        Ok(()) => println!("Rust parser tests passed!"),
        Err(e) => panic!("Rust parser tests failed: {}", e),
    }
}