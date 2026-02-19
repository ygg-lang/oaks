use oak_core::{Lexer, NoLexerCache, Parser, SourceText, parser::ParseSession};
use oak_kotlin::{KotlinLanguage, KotlinLexer, KotlinParser};

#[test]
fn test_data_class_parsing() {
    let source = SourceText::new("data class Person(val name: String, val age: Int) {}");
    let config = KotlinLanguage::new();
    let lexer = KotlinLexer::new(&config);
    let lex_output = lexer.lex(&source, &[], &mut NoLexerCache);

    assert!(lex_output.result.is_ok());

    let parser = KotlinParser::new(&config);
    let mut cache = ParseSession::new(16);
    let parse_output = parser.parse(&source, &[], &mut cache);

    assert!(parse_output.result.is_ok());
}

#[test]
fn test_sealed_class_parsing() {
    let source = SourceText::new("sealed class Result<T> {}");
    let config = KotlinLanguage::new();
    let lexer = KotlinLexer::new(&config);
    let lex_output = lexer.lex(&source, &[], &mut NoLexerCache);

    assert!(lex_output.result.is_ok());

    let parser = KotlinParser::new(&config);
    let mut cache = ParseSession::new(16);
    let parse_output = parser.parse(&source, &[], &mut cache);

    assert!(parse_output.result.is_ok());
}

#[test]
fn test_extension_function_parsing() {
    let source = SourceText::new("fun String.reverse(): String { return this.reversed() }");
    let config = KotlinLanguage::new();
    let lexer = KotlinLexer::new(&config);
    let lex_output = lexer.lex(&source, &[], &mut NoLexerCache);

    assert!(lex_output.result.is_ok());

    let parser = KotlinParser::new(&config);
    let mut cache = ParseSession::new(16);
    let parse_output = parser.parse(&source, &[], &mut cache);

    assert!(parse_output.result.is_ok());
}

#[test]
fn test_suspend_function_parsing() {
    let source = SourceText::new("suspend fun fetchData(): String { return \"data\" }");
    let config = KotlinLanguage::new();
    let lexer = KotlinLexer::new(&config);
    let lex_output = lexer.lex(&source, &[], &mut NoLexerCache);

    assert!(lex_output.result.is_ok());

    let parser = KotlinParser::new(&config);
    let mut cache = ParseSession::new(16);
    let parse_output = parser.parse(&source, &[], &mut cache);

    assert!(parse_output.result.is_ok());
}

#[test]
fn test_inline_function_parsing() {
    let source = SourceText::new("inline fun <T> withLock(lock: Lock, action: () -> T): T { lock.lock(); try { return action() } finally { lock.unlock() } }");
    let config = KotlinLanguage::new();
    let lexer = KotlinLexer::new(&config);
    let lex_output = lexer.lex(&source, &[], &mut NoLexerCache);

    assert!(lex_output.result.is_ok());

    let parser = KotlinParser::new(&config);
    let mut cache = ParseSession::new(16);
    let parse_output = parser.parse(&source, &[], &mut cache);

    assert!(parse_output.result.is_ok());
}

#[test]
fn test_reified_function_parsing() {
    let source = SourceText::new("inline fun <reified T> T.printType() { println(T::class.simpleName) }");
    let config = KotlinLanguage::new();
    let lexer = KotlinLexer::new(&config);
    let lex_output = lexer.lex(&source, &[], &mut NoLexerCache);

    assert!(lex_output.result.is_ok());

    let parser = KotlinParser::new(&config);
    let mut cache = ParseSession::new(16);
    let parse_output = parser.parse(&source, &[], &mut cache);

    assert!(parse_output.result.is_ok());
}

#[test]
fn test_when_expression_parsing() {
    let source = SourceText::new("fun evaluate(x: Int): String = when (x) { 1 -> \"one\"; 2 -> \"two\"; else -> \"other\" }");
    let config = KotlinLanguage::new();
    let lexer = KotlinLexer::new(&config);
    let lex_output = lexer.lex(&source, &[], &mut NoLexerCache);

    assert!(lex_output.result.is_ok());

    let parser = KotlinParser::new(&config);
    let mut cache = ParseSession::new(16);
    let parse_output = parser.parse(&source, &[], &mut cache);

    assert!(parse_output.result.is_ok());
}

#[test]
fn test_if_expression_parsing() {
    let source = SourceText::new("fun max(a: Int, b: Int): Int = if (a > b) a else b");
    let config = KotlinLanguage::new();
    let lexer = KotlinLexer::new(&config);
    let lex_output = lexer.lex(&source, &[], &mut NoLexerCache);

    assert!(lex_output.result.is_ok());

    let parser = KotlinParser::new(&config);
    let mut cache = ParseSession::new(16);
    let parse_output = parser.parse(&source, &[], &mut cache);

    assert!(parse_output.result.is_ok());
}

#[test]
fn test_try_expression_parsing() {
    let source = SourceText::new("fun divide(a: Int, b: Int): Int? = try { a / b } catch (e: ArithmeticException) { null } finally { println(\"Done\") }");
    let config = KotlinLanguage::new();
    let lexer = KotlinLexer::new(&config);
    let lex_output = lexer.lex(&source, &[], &mut NoLexerCache);

    assert!(lex_output.result.is_ok());

    let parser = KotlinParser::new(&config);
    let mut cache = ParseSession::new(16);
    let parse_output = parser.parse(&source, &[], &mut cache);

    assert!(parse_output.result.is_ok());
}

#[test]
fn test_regular_class_parsing() {
    let source = SourceText::new(
        "class MyClass<T>(val value: T) : MyInterface<T> {
    fun doSomething() {}
}",
    );
    let config = KotlinLanguage::new();
    let lexer = KotlinLexer::new(&config);
    let lex_output = lexer.lex(&source, &[], &mut NoLexerCache);
    assert!(lex_output.result.is_ok());

    let parser = KotlinParser::new(&config);
    let mut cache = ParseSession::new(16);
    let parse_output = parser.parse(&source, &[], &mut cache);
    assert!(parse_output.result.is_ok());
}

#[test]
fn test_interface_parsing() {
    let source = SourceText::new(
        "interface MyInterface<T> {
    fun process(value: T): T
}",
    );
    let config = KotlinLanguage::new();
    let lexer = KotlinLexer::new(&config);
    let lex_output = lexer.lex(&source, &[], &mut NoLexerCache);
    assert!(lex_output.result.is_ok());

    let parser = KotlinParser::new(&config);
    let mut cache = ParseSession::new(16);
    let parse_output = parser.parse(&source, &[], &mut cache);
    assert!(parse_output.result.is_ok());
}

#[test]
fn test_object_parsing() {
    let source = SourceText::new(
        "object Singleton {
    val instance = Singleton
}",
    );
    let config = KotlinLanguage::new();
    let lexer = KotlinLexer::new(&config);
    let lex_output = lexer.lex(&source, &[], &mut NoLexerCache);
    assert!(lex_output.result.is_ok());

    let parser = KotlinParser::new(&config);
    let mut cache = ParseSession::new(16);
    let parse_output = parser.parse(&source, &[], &mut cache);
    assert!(parse_output.result.is_ok());
}

#[test]
fn test_companion_object_parsing() {
    let source = SourceText::new(
        "class MyClass {
    companion object Factory {
        fun create(): MyClass = MyClass()
    }
}",
    );
    let config = KotlinLanguage::new();
    let lexer = KotlinLexer::new(&config);
    let lex_output = lexer.lex(&source, &[], &mut NoLexerCache);
    assert!(lex_output.result.is_ok());

    let parser = KotlinParser::new(&config);
    let mut cache = ParseSession::new(16);
    let parse_output = parser.parse(&source, &[], &mut cache);
    assert!(parse_output.result.is_ok());
}

#[test]
fn test_property_parsing() {
    let source = SourceText::new(
        "class MyClass {
    private val constantValue = 42
    public var mutableValue: String = \"default\"
    lateinit var lateInitValue: String
    const val CONSTANT = \"constant\"
}",
    );
    let config = KotlinLanguage::new();
    let lexer = KotlinLexer::new(&config);
    let lex_output = lexer.lex(&source, &[], &mut NoLexerCache);
    assert!(lex_output.result.is_ok());

    let parser = KotlinParser::new(&config);
    let mut cache = ParseSession::new(16);
    let parse_output = parser.parse(&source, &[], &mut cache);
    assert!(parse_output.result.is_ok());
}

#[test]
fn test_null_safety_parsing() {
    let source = SourceText::new(
        "fun processValue(value: String?) {
    val length = value?.length ?: 0
    println(length)
}",
    );
    let config = KotlinLanguage::new();
    let lexer = KotlinLexer::new(&config);
    let lex_output = lexer.lex(&source, &[], &mut NoLexerCache);
    assert!(lex_output.result.is_ok());

    let parser = KotlinParser::new(&config);
    let mut cache = ParseSession::new(16);
    let parse_output = parser.parse(&source, &[], &mut cache);
    assert!(parse_output.result.is_ok());
}

#[test]
fn test_error_recovery() {
    let source = SourceText::new(
        "class MyClass {
    fun brokenFunction() {
        // Missing closing brace

    val validProperty = 42
}",
    );
    let config = KotlinLanguage::new();
    let lexer = KotlinLexer::new(&config);
    let lex_output = lexer.lex(&source, &[], &mut NoLexerCache);
    assert!(lex_output.result.is_ok());

    let parser = KotlinParser::new(&config);
    let mut cache = ParseSession::new(16);
    let parse_output = parser.parse(&source, &[], &mut cache);
    assert!(parse_output.result.is_ok());
}
