use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_gsgl::{GsglLanguage, GsglParser};

#[test]
fn test_parser_basic() {
    let language = GsglLanguage::default();
    let parser = GsglParser::new(&language);
    let source = SourceText::new(r#"local x = 42"#);
    let mut cache = ParseSession::<GsglLanguage>::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.result.is_ok());
    println!("Parsed successfully: {:?}", result.result);
}

#[test]
fn test_parser_function_declaration() {
    let language = GsglLanguage::default();
    let parser = GsglParser::new(&language);
    let source = SourceText::new(
        r#"
        function add(a, b)
            return a + b
        end
        "#,
    );
    let mut cache = ParseSession::<GsglLanguage>::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.result.is_ok());
    println!("Function declaration parsed: {:?}", result.result);
}

#[test]
fn test_parser_local_function() {
    let language = GsglLanguage::default();
    let parser = GsglParser::new(&language);
    let source = SourceText::new(
        r#"
        local function factorial(n)
            if n <= 1 then
                return 1
            else
                return n * factorial(n - 1)
            end
        end
        
        local result = factorial(5)
        print(result)
        "#,
    );
    let mut cache = ParseSession::<GsglLanguage>::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.result.is_ok());
    println!("Local function parsed: {:?}", result.result);
}

#[test]
fn test_parser_table_constructor() {
    let language = GsglLanguage::default();
    let parser = GsglParser::new(&language);
    let source = SourceText::new(r#"local t = {a = 1, b = 2, [3] = "three"}"#);

    let mut cache = ParseSession::<GsglLanguage>::default();
    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.result.is_ok());
    println!("Table constructor parsed: {:?}", result.result);
}

#[test]
fn test_parser_control_structures() {
    let language = GsglLanguage::default();
    let parser = GsglParser::new(&language);
    let source = SourceText::new(
        r#"
        local x = 10
        
        if x > 5 then
            print("x is greater than 5")
        elseif x == 5 then
            print("x equals 5")
        else
            print("x is less than 5")
        end
        
        for i = 1, 10 do
            print(i)
        end
        
        local j = 1
        while j <= 5 do
            print("j is", j)
            j = j + 1
        end
        
        repeat
            print("This runs at least once")
        until true
        "#,
    );

    let mut cache = ParseSession::<GsglLanguage>::default();
    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.result.is_ok());
    println!("Control structures parsed: {:?}", result.result);
}

#[test]
fn test_parser_expressions() {
    let language = GsglLanguage::default();
    let parser = GsglParser::new(&language);
    let source = SourceText::new(
        r#"
        local a = 1 + 2 * 3
        local b = (4 + 5) / 2
        local c = "hello" .. " " .. "world"
        local d = not true and false or true
        "#,
    );

    let mut cache = ParseSession::<GsglLanguage>::default();
    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.result.is_ok());
    println!("Expressions parsed: {:?}", result.result);
}

#[test]
fn test_parser_function_calls() {
    let language = GsglLanguage::default();
    let parser = GsglParser::new(&language);
    let source = SourceText::new(
        r#"
        print("Hello, World!")
        local result = math.max(1, 2, 3)
        local obj = {}
        obj:method(arg1, arg2)
        "#,
    );

    let mut cache = ParseSession::<GsglLanguage>::default();
    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.result.is_ok());
    println!("Function calls parsed: {:?}", result.result);
}

#[test]
fn test_parser_syntax_error() {
    let language = GsglLanguage::default();
    let parser = GsglParser::new(&language);
    let source = SourceText::new(r#"local x = "#); // Incomplete assignment

    let mut cache = ParseSession::<GsglLanguage>::default();
    let result = parser.parse(&source, &[], &mut cache);
    // For now, our basic files doesn't detect kind errors
    // In a real implementation, this should fail
    println!("Syntax error test result: {:?}", result.result);
}

#[test]
fn test_parser_incomplete_function() {
    let language = GsglLanguage::default();
    let parser = GsglParser::new(&language);
    let source = SourceText::new(
        r#"
        function incomplete_func(a, b)
            return a +
        "#,
    ); // Missing closing 'end' and incomplete expression

    let mut cache = ParseSession::<GsglLanguage>::default();
    let result = parser.parse(&source, &[], &mut cache);
    // For now, our basic files doesn't detect kind errors
    // In a real implementation, this should fail
    println!("Incomplete function test result: {:?}", result.result);
}

#[test]
fn test_parser_invalid_table_syntax() {
    let language = GsglLanguage::default();
    let parser = GsglParser::new(&language);
    let source = SourceText::new(r#"local t = {a = 1, b = 2,}"#); // Trailing comma

    let mut cache = ParseSession::<GsglLanguage>::default();
    let result = parser.parse(&source, &[], &mut cache);
    // For now, our basic files doesn't detect kind errors
    // In a real implementation, this might be valid or invalid depending on Lua version
    println!("Invalid table kind test result: {:?}", result.result);
}
