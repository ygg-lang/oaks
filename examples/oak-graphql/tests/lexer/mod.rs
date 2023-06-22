use oak_core::{LexerState, helpers::LexerTester, source::Source};
use oak_graphql::{GraphQLLanguage, GraphQLLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_graphql_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(GraphQLLanguage::default()));
    let lexer = GraphQLLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer/fixtures")).with_extension("graphql").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<GraphQLLanguage, _>(&lexer) {
        Ok(()) => println!("GraphQL lexer tests passed!"),
        Err(e) => panic!("GraphQL lexer tests failed: {}", e),
    }
}

#[test]
fn test_peek_behavior() {
    use oak_core::{LexerState, SourceText, lexer::LexerState};
    use oak_graphql::GraphQLLanguage;

    let source = SourceText::new("NESTED_CONSTANT");
    let mut state = LexerState::<&SourceText, GraphQLLanguage>::new(&source);

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
fn test_graphql_query_parsing() {
    use oak_core::{Lexer, LexerState, SourceText};
    use oak_graphql::{GraphQLLanguage, GraphQLLexer};

    let source = SourceText::new("query { user { name } }");
    let language = Box::leak(Box::new(GraphQLLanguage::default()));
    let lexer = GraphQLLexer::new(language);

    let result = lexer.lex(&source);

    println!("测试 GraphQL 查询解析:");
    println!("源代码: '{}'", (&source).get_text_from(0));

    let tokens = result.result.expect("词法分析应该成功");
    assert!(!tokens.is_empty(), "应该解析出至少一个标记");

    let first_token = &tokens[0];
    let source_ref = &source;
    let token_text = source_ref.get_text_in(first_token.span.clone());

    println!("第一个标记: 类型={:?}, 文本='{}'", first_token.kind, token_text);

    println!("✅ GraphQL 查询解析测试通过！");
}