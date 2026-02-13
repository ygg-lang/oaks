use oak_core::{SourceText, builder::Builder, ParseSession, source::ToSource};
use oak_sql::{SqlLanguage, SqlBuilder, ast::*};
#[cfg(feature = "oak-pretty-print")]
use oak_pretty_print::AsDocument;

#[test]
fn test_sql_to_source() {
    let config = SqlLanguage::default();
    let builder = SqlBuilder::new(&config);
    let input = "SELECT name, age FROM users WHERE age > 18;";
    let source = SourceText::new(input);

    let mut session = ParseSession::<SqlLanguage>::default();
    let result = builder.build(&source, &[], &mut session);
    
    assert!(result.result.is_ok());
    let root = result.result.unwrap();
    
    let generated = root.to_source_string();
    // ToSource adds spaces between tokens, and we expect it to be similar to input but maybe not identical in spacing
    assert!(generated.contains("SELECT"));
    assert!(generated.contains("name , age")); // SourceBuffer might add space before comma if not careful
    assert!(generated.contains("FROM users"));
    assert!(generated.contains("WHERE age > 18"));
}

#[test]
#[cfg(feature = "oak-pretty-print")]
fn test_sql_to_doc() {
    let config = SqlLanguage::default();
    let builder = SqlBuilder::new(&config);
    let input = "SELECT name, age FROM users WHERE age > 18;";
    let source = SourceText::new(input);

    let mut session = ParseSession::<SqlLanguage>::default();
    let result = builder.build(&source, &[], &mut session);
    
    assert!(result.result.is_ok());
    let root = result.result.unwrap();
    
    let doc = root.as_document();
    let formatted = doc.render(oak_pretty_print::FormatConfig::default());
    
    assert!(formatted.contains("SELECT"));
    assert!(formatted.contains("FROM users"));
    assert!(formatted.contains("WHERE age > 18"));
}

#[test]
fn test_sql_insert_to_source() {
    let config = SqlLanguage::default();
    let builder = SqlBuilder::new(&config);
    let input = "INSERT INTO users (name, age) VALUES ('John', 25);";
    let source = SourceText::new(input);

    let mut session = ParseSession::<SqlLanguage>::default();
    let result = builder.build(&source, &[], &mut session);
    
    assert!(result.result.is_ok());
    let root = result.result.unwrap();
    
    let generated = root.to_source_string();
    assert!(generated.contains("INSERT INTO users"));
    assert!(generated.contains("VALUES ( 'John' , 25 )"));
}

#[test]
fn test_sql_create_to_source() {
    let config = SqlLanguage::default();
    let builder = SqlBuilder::new(&config);
    let input = "CREATE TABLE users (id INT PRIMARY KEY, name TEXT);";
    let source = SourceText::new(input);

    let mut session = ParseSession::<SqlLanguage>::default();
    let result = builder.build(&source, &[], &mut session);
    
    assert!(result.result.is_ok());
    let root = result.result.unwrap();
    
    let generated = root.to_source_string();
    assert!(generated.contains("CREATE TABLE users"));
    assert!(generated.contains("id INT PRIMARY KEY"));
    assert!(generated.contains("name TEXT"));
}
