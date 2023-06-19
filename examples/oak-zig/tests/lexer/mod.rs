#![feature(new_range_api)]

use oak_core::{helpers::LexerTester, source::Source};
use oak_zig::{ZigLanguage, ZigLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_zig_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(ZigLanguage::default()));
    let lexer = ZigLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer"))
        .with_extension("zig")
        .with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<ZigLanguage, _>(lexer) {
        Ok(()) => println!("Zig lexer tests passed!"),
        Err(e) => panic!("Zig lexer tests failed: {}", e),
    }
}

#[test]
fn test_peek_behavior() {
    use oak_core::{SourceText, lexer::LexerState};
    use oak_zig::ZigLanguage;

    let source = SourceText::new("const x = 42;");
    let mut state = LexerState::<&SourceText, ZigLanguage>::new(&source);

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
fn test_zig_identifier_parsing() {
    use oak_core::{Lexer, SourceText};
    use oak_zig::{ZigLanguage, ZigLexer, ZigSyntaxKind};

    let source = SourceText::new("const identifier = 42;");
    let language = ZigLanguage::default();
    let lexer = ZigLexer::new(&language);
    let output = lexer.lex_incremental(&source, 0, Default::default());

    println!("Tokens:");
    for token in output.tokens {
        println!("  {:?}: {:?}", token.kind, token.text(&source));
    }

    // 验证第一个token是const关键字
    assert_eq!(output.tokens[0].kind, ZigSyntaxKind::Const);
    // 验证第二个token是标识符
    assert_eq!(output.tokens[2].kind, ZigSyntaxKind::Identifier);
}

#[test]
fn test_zig_number_parsing() {
    use oak_core::{Lexer, SourceText};
    use oak_zig::{ZigLanguage, ZigLexer, ZigSyntaxKind};

    let source = SourceText::new("42 3.14 0x1A 0b1010 0o777");
    let language = ZigLanguage::default();
    let lexer = ZigLexer::new(&language);
    let output = lexer.lex_incremental(&source, 0, Default::default());

    println!("Number tokens:");
    for token in &output.tokens {
        if matches!(token.kind, ZigSyntaxKind::IntegerLiteral | ZigSyntaxKind::FloatLiteral) {
            println!("  {:?}: {:?}", token.kind, token.text(&source));
        }
    }

    // 验证数字类型
    let number_tokens: Vec<_> = output.tokens.iter()
        .filter(|t| matches!(t.kind, ZigSyntaxKind::IntegerLiteral | ZigSyntaxKind::FloatLiteral))
        .collect();
    
    assert_eq!(number_tokens[0].kind, ZigSyntaxKind::IntegerLiteral); // 42
    assert_eq!(number_tokens[1].kind, ZigSyntaxKind::FloatLiteral);   // 3.14
    assert_eq!(number_tokens[2].kind, ZigSyntaxKind::IntegerLiteral); // 0x1A
    assert_eq!(number_tokens[3].kind, ZigSyntaxKind::IntegerLiteral); // 0b1010
    assert_eq!(number_tokens[4].kind, ZigSyntaxKind::IntegerLiteral); // 0o777
}