use oak_fluent::parser::parse;

#[test]
fn test_parse_basic_ftl() {
    let ftl = r#"messages.hello = Hello world!
messages.welcome = Welcome to iTools!"#;

    match parse(ftl) {
        Ok(ast) => {
            println!("Parsed successfully: {:?}", ast);
            assert_eq!(ast.messages.len(), 2);
        }
        Err(err) => {
            println!("Parse error: {:?}", err);
            panic!("Failed to parse FTL content: {:?}", err);
        }
    }
}

#[test]
fn test_parse_ftl_with_args() {
    let ftl = r#"messages.greeting = Hello, {name}!
messages.item_count = You have {count} items."#;

    match parse(ftl) {
        Ok(ast) => {
            println!("Parsed successfully: {:?}", ast);
            assert_eq!(ast.messages.len(), 2);
        }
        Err(err) => {
            println!("Parse error: {:?}", err);
            panic!("Failed to parse FTL content: {:?}", err);
        }
    }
}

#[test]
fn test_parse_ftl_with_plural() {
    let ftl = r#"messages.item_count = { $count ->
        [one] There is { $count } item
        *[other] There are { $count } items
    }"#;

    match parse(ftl) {
        Ok(ast) => {
            println!("Parsed successfully: {:?}", ast);
            assert_eq!(ast.messages.len(), 1);
        }
        Err(err) => {
            println!("Parse error: {:?}", err);
            panic!("Failed to parse FTL content: {:?}", err);
        }
    }
}
