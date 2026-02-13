use oak_core::Builder;
use oak_sql::{SqlBuilder, SqlLanguage};

#[test]
fn test_select_ast() {
    let language = SqlLanguage::default();
    let builder = SqlBuilder::new(&language);
    let mut cache = oak_core::parser::session::ParseSession::default();

    let source = "SELECT id, name FROM users WHERE age > 18;";
    let result = builder.build(source, &[], &mut cache);

    if let Err(e) = &result.result {
        panic!("Build failed: {:?}", e);
    }
    let root = result.result.unwrap();
    assert_eq!(root.statements.len(), 1);

    if let oak_sql::ast::SqlStatement::Select(select) = &root.statements[0] {
        assert_eq!(select.items.len(), 2);
        assert!(select.from.is_some());
        assert!(select.selection.is_some());
    }
    else {
        panic!("Expected Select statement");
    }
}

#[test]
fn test_create_table_ast() {
    let language = SqlLanguage::default();
    let builder = SqlBuilder::new(&language);
    let mut cache = oak_core::parser::session::ParseSession::default();

    let source = "CREATE TABLE users (id INT PRIMARY KEY, name VARCHAR(255) NOT NULL);";
    let result = builder.build(source, &[], &mut cache);

    assert!(result.result.is_ok());
    let root = result.result.unwrap();
    assert_eq!(root.statements.len(), 1);

    if let oak_sql::ast::SqlStatement::Create(create) = &root.statements[0] {
        assert_eq!(create.name.name, "users");
        assert_eq!(create.columns.len(), 2);
        assert_eq!(create.columns[0].name.name, "id");
        assert_eq!(create.columns[0].data_type, "INT");
        assert_eq!(create.columns[1].name.name, "name");
        assert_eq!(create.columns[1].data_type, "VARCHAR(255)");
    }
    else {
        panic!("Expected Create statement");
    }
}
