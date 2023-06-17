use oak_core::{Parser, SourceText, errors::OakErrorKind};
use oak_lua::{LuaLanguage, LuaParser};

#[test]
fn test_parser_basic() {
    let language = LuaLanguage::new();
    let parser = LuaParser::new(&language);
    let source = SourceText::new(r#"local x = 42"#);

    let result = parser.parse(&source);
    assert!(result.result.is_ok());
    println!("Parsed successfully: {:?}", result.result);
}

#[test]
fn test_parser_function_declaration() {
    let language = LuaLanguage::new();
    let parser = LuaParser::new(&language);
    let source = SourceText::new(
        r#"
function add(a, b)
    return a + b
end
"#,
    );

    let result = parser.parse(&source);
    assert!(result.result.is_ok());
    println!("Parsed function declaration: {:?}", result.result);
}

#[test]
fn test_parser_local_function() {
    let language = LuaLanguage::new();
    let parser = LuaParser::new(&language);
    let source = SourceText::new(
        r#"
local function factorial(n)
    if n <= 1 then
        return 1
    else
        return n * factorial(n - 1)
    end
end
"#,
    );

    let result = parser.parse(&source);
    assert!(result.result.is_ok());
    println!("Parsed local function: {:?}", result.result);
}

#[test]
fn test_parser_table_constructor() {
    let language = LuaLanguage::new();
    let parser = LuaParser::new(&language);
    let source = SourceText::new(
        r#"
local t = {
    name = "John",
    age = 30,
    [1] = "first",
    ["key"] = "value"
}
"#,
    );

    let result = parser.parse(&source);
    assert!(result.result.is_ok());
    println!("Parsed table constructor: {:?}", result.result);
}

#[test]
fn test_parser_control_structures() {
    let language = LuaLanguage::new();
    let parser = LuaParser::new(&language);
    let source = SourceText::new(
        r#"
if x > 0 then
    print("positive")
elseif x < 0 then
    print("negative")
else
    print("zero")
end

while i < 10 do
    i = i + 1
end

for i = 1, 10 do
    print(i)
end

for k, v in pairs(t) do
    print(k, v)
end

repeat
    x = x + 1
until x > 100
"#,
    );

    let result = parser.parse(&source);
    assert!(result.result.is_ok());
    println!("Parsed control structures: {:?}", result.result);
}

#[test]
fn test_parser_expressions() {
    let language = LuaLanguage::new();
    let parser = LuaParser::new(&language);
    let source = SourceText::new(
        r#"
local result = (a + b) * c / d - e % f ^ g
local bool_result = x and y or z
local comparison = a == b and c ~= d and e < f and g > h
local string_concat = "hello" .. " " .. "world"
"#,
    );

    let result = parser.parse(&source);
    assert!(result.result.is_ok());
    println!("Parsed expressions: {:?}", result.result);
}

#[test]
fn test_parser_function_calls() {
    let language = LuaLanguage::new();
    let parser = LuaParser::new(&language);
    let source = SourceText::new(
        r#"
print("hello")
math.max(1, 2, 3)
obj:method(arg1, arg2)
func{key = "value"}
func"string argument"
"#,
    );

    let result = parser.parse(&source);
    assert!(result.result.is_ok());
    println!("Parsed function calls: {:?}", result.result);
}

#[test]
fn test_parser_syntax_error() {
    let language = LuaLanguage::new();
    let parser = LuaParser::new(&language);
    let source = SourceText::new(r#"local x = "#);

    let result = parser.parse(&source);
    // 应该有语法错
    assert!(!result.diagnostics.is_empty());
    println!("Syntax error detected: {:?}", result.diagnostics);
}

#[test]
fn test_parser_incomplete_function() {
    let language = LuaLanguage::new();
    let parser = LuaParser::new(&language);
    let source = SourceText::new(
        r#"
function incomplete(a, b)
    return a + b
-- missing 'end'
"#,
    );

    let result = parser.parse(&source);
    // 应该检测到不完整的函数定义
    assert!(!result.diagnostics.is_empty());
    println!("Incomplete function error: {:?}", result.diagnostics);
}

#[test]
fn test_parser_invalid_table_syntax() {
    let language = LuaLanguage::new();
    let parser = LuaParser::new(&language);
    let source = SourceText::new(
        r#"
local t = {
    key = value,
    [invalid key] = "value"
}
"#,
    );

    let result = parser.parse(&source);
    // 应该检测到无效的表语法
    assert!(!result.diagnostics.is_empty());
    println!("Invalid table kind error: {:?}", result.diagnostics);
}
