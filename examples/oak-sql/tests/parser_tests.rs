use oak_core::{SourceText, builder::Builder, parser::session::ParseSession};
use oak_sql::{SqlLanguage, SqlBuilder, ast::*};

#[test]
fn test_sql_builder_select() {
    let config = SqlLanguage::default();
    let builder = SqlBuilder::new(&config);
    let source = SourceText::new("SELECT name, age FROM users WHERE age > 18;");

    let mut session = oak_core::builder::session::BuilderSession::<SqlLanguage>::default();
    let result = builder.build(&source, &[], &mut session);
    
    assert!(result.result.is_ok());
    let root = result.result.unwrap();
    assert_eq!(root.statements.len(), 1);
    
    if let Statement::Select(select) = &root.statements[0] {
        assert_eq!(select.select_items.len(), 2);
        assert!(select.from.is_some());
        assert_eq!(select.from.as_ref().unwrap().name, "users");
    } else {
        panic!("Expected Select statement");
    }
}

#[test]
fn test_sql_builder_insert() {
    let config = SqlLanguage::default();
    let builder = SqlBuilder::new(&config);
    let source = SourceText::new("INSERT INTO users (name, age) VALUES ('John', 25);");

    let mut session = oak_core::builder::session::BuilderSession::<SqlLanguage>::default();
    let result = builder.build(&source, &[], &mut session);
    
    assert!(result.result.is_ok());
    let root = result.result.unwrap();
    assert_eq!(root.statements.len(), 1);
    
    if let Statement::Insert(insert) = &root.statements[0] {
        assert_eq!(insert.table.name, "users");
        assert_eq!(insert.columns.len(), 2);
        assert_eq!(insert.values.len(), 1);
        assert_eq!(insert.values[0].len(), 2);
    } else {
        panic!("Expected Insert statement");
    }
}

#[test]
fn test_sql_builder_create_table() {
    let config = SqlLanguage::default();
    let builder = SqlBuilder::new(&config);
    let source = SourceText::new("CREATE TABLE users (id INT PRIMARY KEY, name TEXT);");

    let mut session = oak_core::builder::session::BuilderSession::<SqlLanguage>::default();
    let result = builder.build(&source, &[], &mut session);
    
    assert!(result.result.is_ok());
    let root = result.result.unwrap();
    assert_eq!(root.statements.len(), 1);
    
    if let Statement::Create(create) = &root.statements[0] {
        assert_eq!(create.object_type, "TABLE");
        assert_eq!(create.name, "users");
        assert_eq!(create.columns.len(), 2);
        assert_eq!(create.columns[0].name, "id");
        assert_eq!(create.columns[1].name, "name");
    } else {
        panic!("Expected Create statement");
    }
}
