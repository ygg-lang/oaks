use oak_core::{helpers::LexerTester, source::Source};
use oak_fortran::{FortranLanguage, FortranLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_fortran_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(FortranLanguage::default()));
    let lexer = FortranLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer/fixtures")).with_extension("f90").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<FortranLanguage, _>(&lexer) {
        Ok(()) => println!("Fortran lexer tests passed!"),
        Err(e) => panic!("Fortran lexer tests failed: {}", e),
    }
}

#[test]
fn test_peek_behavior() {
    use oak_core::{LexerState, SourceText, parser::session::ParseSession};
    use oak_fortran::FortranLanguage;

    let source = SourceText::new("NESTED_CONSTANT");
    let _cache = ParseSession::<FortranLanguage>::default();
    let mut state = LexerState::<SourceText, FortranLanguage>::new(&source);

    println!("初始状态:");
    println!("位置: {}", state.get_position());
    println!("current(): {:?}", state.current());
    println!("peek(): {:?}", state.peek());

    println!("\n前进 1 个字符后:");
    state.advance(1);
    println!("位置: {}", state.get_position());
    println!("current(): {:?}", state.current());
    println!("peek(): {:?}", state.peek());

    println!("\n前进 1 个字符后:");
    state.advance(1);
    println!("位置: {}", state.get_position());
    println!("current(): {:?}", state.current());
    println!("peek(): {:?}", state.peek());
}

#[test]
fn test_fortran_program_parsing() {
    use oak_core::{Lexer, SourceText, parser::session::ParseSession};
    use oak_fortran::{FortranLanguage, FortranLexer};

    let source = SourceText::new("program hello\n  print *, 'Hello, World!'\nend program hello");
    let language = Box::leak(Box::new(FortranLanguage::default()));
    let lexer = FortranLexer::new(language);

    let mut cache = ParseSession::<FortranLanguage>::default();
    let result = lexer.lex(&source, &[], &mut cache);

    println!("测试 Fortran 程序解析:");
    println!("源代码: '{}'", (&source).get_text_from(0));

    let tokens = result.result.expect("词法分析应该成功");
    assert!(!tokens.is_empty(), "应该解析出至少一个标记");

    let first_token = &tokens[0];
    let source_ref = &source;
    let token_text = source_ref.get_text_in(first_token.span.clone());

    println!("第一个标记: 类型={:?}, 文本='{}'", first_token.kind, token_text);

    println!("✅ Fortran 程序解析测试通过！");
}
