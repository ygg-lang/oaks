use oak_core::{GreenBuilder, IncrementalCache, Lexer, LexerState, SourceText};
use oak_crystal::{CrystalLanguage, CrystalLexer};

#[test]
fn test_crystal_lexer_basic() {
    println!("Testing Crystal Lexer - Basic...");

    let language = CrystalLanguage::new();
    let lexer = CrystalLexer::new(&language);
    let source = SourceText::new("class Test\n  def hello\n    puts \"Hello, World!\"\n  end\nend");
    let mut builder = GreenBuilder::new(0);
    let cache = IncrementalCache::new(&mut builder);

    let result = lexer.lex_incremental(&source, 0, cache);

    if result.diagnostics.is_empty() {
        println!("  ✓ Basic Crystal code lexed successfully");
    }
    else {
        println!("  ✗ Lexer produced errors: {:?}", result.diagnostics);
    }
}

#[test]
fn test_crystal_lexer_keywords() {
    println!("Testing Crystal Lexer - Keywords...");

    let language = CrystalLanguage::new();
    let lexer = CrystalLexer::new(&language);
    let source = SourceText::new("class module def end if else elsif unless case when");
    let mut builder = GreenBuilder::new(0);
    let cache = IncrementalCache::new(&mut builder);

    let result = lexer.lex_incremental(&source, 0, cache);

    if result.diagnostics.is_empty() {
        println!("  ✓ Crystal keywords lexed successfully");
    }
    else {
        println!("  ✗ Lexer produced errors: {:?}", result.diagnostics);
    }
}

#[test]
fn test_crystal_lexer_operators() {
    println!("Testing Crystal Lexer - Operators...");

    let language = CrystalLanguage::new();
    let lexer = CrystalLexer::new(&language);
    let source = SourceText::new("+ - * / % == != < > <= >= && || ! & | ^");
    let mut builder = GreenBuilder::new(0);
    let cache = IncrementalCache::new(&mut builder);

    let result = lexer.lex_incremental(&source, 0, cache);

    if result.diagnostics.is_empty() {
        println!("  ✓ Crystal operators lexed successfully");
    }
    else {
        println!("  ✗ Lexer produced errors: {:?}", result.diagnostics);
    }
}

#[test]
fn test_crystal_lexer_strings() {
    println!("Testing Crystal Lexer - Strings...");

    let language = CrystalLanguage::new();
    let lexer = CrystalLexer::new(&language);
    let source = SourceText::new("\"hello\" 'world' \"escaped \\\"string\\\"\"");
    let mut builder = GreenBuilder::new(0);
    let cache = IncrementalCache::new(&mut builder);

    let result = lexer.lex_incremental(&source, 0, cache);

    if result.diagnostics.is_empty() {
        println!("  ✓ Crystal strings lexed successfully");
    }
    else {
        println!("  ✗ Lexer produced errors: {:?}", result.diagnostics);
    }
}

#[test]
fn test_crystal_lexer_numbers() {
    println!("Testing Crystal Lexer - Numbers...");

    let language = CrystalLanguage::new();
    let lexer = CrystalLexer::new(&language);
    let source = SourceText::new("123 456.789 1_000_000");
    let mut builder = GreenBuilder::new(0);
    let cache = IncrementalCache::new(&mut builder);

    let result = lexer.lex_incremental(&source, 0, cache);

    if result.diagnostics.is_empty() {
        println!("  ✓ Crystal numbers lexed successfully");
    }
    else {
        println!("  ✗ Lexer produced errors: {:?}", result.diagnostics);
    }
}

#[test]
fn test_crystal_lexer_comments() {
    println!("Testing Crystal Lexer - Comments...");

    let language = CrystalLanguage::new();
    let lexer = CrystalLexer::new(&language);
    let source = SourceText::new("# This is a comment\nclass Test # Another comment\nend");
    let mut builder = GreenBuilder::new(0);
    let cache = IncrementalCache::new(&mut builder);

    let result = lexer.lex_incremental(&source, 0, cache);

    if result.diagnostics.is_empty() {
        println!("  ✓ Crystal comments lexed successfully");
    }
    else {
        println!("  ✗ Lexer produced errors: {:?}", result.diagnostics);
    }
}

fn main() {
    println!("Running Crystal Language Tests");
    println!("==============================");

    test_crystal_lexer_basic();
    test_crystal_lexer_keywords();
    test_crystal_lexer_operators();
    test_crystal_lexer_strings();
    test_crystal_lexer_numbers();
    test_crystal_lexer_comments();

    println!("\nAll Crystal tests completed!");
}
