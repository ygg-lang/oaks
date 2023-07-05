use oak_core::{ParseSession, SourceText, builder::Builder};
use oak_sql::{SqlBuilder, SqlLanguage, ast::*};

#[test]
fn test_sql_builder_select() {
    let config = SqlLanguage::default();
    let builder = SqlBuilder::new(&config);
    let source = SourceText::new("SELECT name, age FROM users WHERE age > 18;");

    let mut session = ParseSession::<SqlLanguage>::default();
    let result = builder.build(&source, &[], &mut session);

    if let Err(e) = &result.result {
        panic!("Build error: {e:?}\nDiagnostics: {:?}", result.diagnostics);
    }
    let root = result.result.unwrap();
    assert_eq!(root.statements.len(), 1);

    let SqlStatement::Select(select) = &root.statements[0]
    else {
        panic!("Expected Select statement");
    };
    assert_eq!(select.items.len(), 2);
    assert!(select.from.is_some());
    assert!(select.expr.is_some());
}

#[test]
fn test_sql_builder_insert() {
    let config = SqlLanguage::default();
    let builder = SqlBuilder::new(&config);
    let source = SourceText::new("INSERT INTO users (name, age) VALUES ('John', 25);");

    let mut session = ParseSession::<SqlLanguage>::default();
    let result = builder.build(&source, &[], &mut session);

    if let Err(e) = &result.result {
        panic!("Build error: {e:?}\nDiagnostics: {:?}", result.diagnostics);
    }
    let root = result.result.unwrap();
    assert_eq!(root.statements.len(), 1);

    let SqlStatement::Insert(insert) = &root.statements[0]
    else {
        panic!("Expected Insert statement");
    };
    assert_eq!(insert.table_name.name.name.as_ref(), "users");
    assert_eq!(insert.columns.len(), 2);
    assert_eq!(insert.values.len(), 2);
}

#[test]
fn test_sql_builder_create_table() {
    let config = SqlLanguage::default();
    let builder = SqlBuilder::new(&config);
    let source = SourceText::new("CREATE TABLE users (id INT PRIMARY KEY, name TEXT);");

    let mut session = ParseSession::<SqlLanguage>::default();
    let result = builder.build(&source, &[], &mut session);

    assert!(result.result.is_ok());
    let root = result.result.unwrap();
    assert_eq!(root.statements.len(), 1);

    let SqlStatement::Create(create) = &root.statements[0]
    else {
        panic!("Expected Create statement");
    };
    assert_eq!(create.object_type, CreateObjectType::Table);
    assert_eq!(create.name.name.as_ref(), "users");
    let CreateBody::Table { columns, .. } = &create.body
    else {
        panic!("Expected Table body");
    };
    assert_eq!(columns.len(), 2);
    assert_eq!(columns[0].name.name.as_ref(), "id");
    assert_eq!(columns[0].data_type.as_ref(), "INT");
    assert!(!columns[0].constraints.is_empty());
    assert_eq!(columns[1].name.name.as_ref(), "name");
    assert_eq!(columns[1].data_type.as_ref(), "TEXT");
}

#[test]
fn test_sql_builder_create_view() {
    let config = SqlLanguage::default();
    let builder = SqlBuilder::new(&config);
    let source = SourceText::new("CREATE VIEW active_users AS SELECT name FROM users WHERE active = 1;");

    let mut session = ParseSession::<SqlLanguage>::default();
    let result = builder.build(&source, &[], &mut session);

    assert!(result.result.is_ok());
    let root = result.result.unwrap();

    let SqlStatement::Create(create) = &root.statements[0]
    else {
        panic!("Expected Create statement");
    };
    assert_eq!(create.object_type, CreateObjectType::View);
    assert_eq!(create.name.name.as_ref(), "active_users");
    let CreateBody::View { query, .. } = &create.body
    else {
        panic!("Expected View body");
    };
    assert!(query.from.is_some());
    assert!(query.expr.is_some());
}

#[test]
fn test_sql_builder_update_delete() {
    let config = SqlLanguage::default();
    let builder = SqlBuilder::new(&config);
    let mut session = ParseSession::<SqlLanguage>::default();

    let source_update = SourceText::new("UPDATE users SET name = 'John', age = 30 WHERE id = 1;");
    let result_update = builder.build(&source_update, &[], &mut session);
    assert!(result_update.result.is_ok());
    let root_update = result_update.result.unwrap();
    let SqlStatement::Update(update) = &root_update.statements[0]
    else {
        panic!("Expected Update statement");
    };
    assert_eq!(update.table_name.name.name.as_ref(), "users");
    assert_eq!(update.assignments.len(), 2);
    assert!(update.selection.is_some());

    let source_delete = SourceText::new("DELETE FROM users WHERE id = 1;");
    let result_delete = builder.build(&source_delete, &[], &mut session);
    assert!(result_delete.result.is_ok());
    let root_delete = result_delete.result.unwrap();
    let SqlStatement::Delete(delete) = &root_delete.statements[0]
    else {
        panic!("Expected Delete statement");
    };
    assert_eq!(delete.table_name.name.name.as_ref(), "users");
    assert!(delete.selection.is_some());
}

#[test]
fn test_sql_builder_drop_alter() {
    let config = SqlLanguage::default();
    let builder = SqlBuilder::new(&config);
    let mut session = ParseSession::<SqlLanguage>::default();

    let source_drop = SourceText::new("DROP TABLE IF EXISTS users;");
    let result_drop = builder.build(&source_drop, &[], &mut session);
    assert!(result_drop.result.is_ok());
    let root_drop = result_drop.result.unwrap();
    let SqlStatement::Drop(drop) = &root_drop.statements[0]
    else {
        panic!("Expected Drop statement");
    };
    assert_eq!(drop.object_type, DropObjectType::Table);
    assert_eq!(drop.name.name.as_ref(), "users");
    assert!(drop.if_exists);

    let source_alter = SourceText::new("ALTER TABLE users RENAME TO customers;");
    let result_alter = builder.build(&source_alter, &[], &mut session);
    assert!(result_alter.result.is_ok());
    let root_alter = result_alter.result.unwrap();
    let SqlStatement::Alter(alter) = &root_alter.statements[0]
    else {
        panic!("Expected Alter statement");
    };
    assert_eq!(alter.table_name.name.name.as_ref(), "users");
}

#[test]
fn test_sql_builder_database_stmt() {
    let config = SqlLanguage::default();
    let builder = SqlBuilder::new(&config);
    let mut session = ParseSession::<SqlLanguage>::default();

    let source_create = SourceText::new("CREATE DATABASE test_db;");
    let result_create = builder.build(&source_create, &[], &mut session);
    assert!(result_create.result.is_ok());
    let root_create = result_create.result.unwrap();
    let SqlStatement::Create(create) = &root_create.statements[0]
    else {
        panic!("Expected Create statement");
    };
    assert_eq!(create.object_type, CreateObjectType::Database);
    assert_eq!(create.name.name.as_ref(), "test_db");
    assert!(matches!(create.body, CreateBody::Database { .. }));

    let source_drop = SourceText::new("DROP DATABASE test_db;");
    let result_drop = builder.build(&source_drop, &[], &mut session);
    assert!(result_drop.result.is_ok());
    let root_drop = result_drop.result.unwrap();
    let SqlStatement::Drop(drop) = &root_drop.statements[0]
    else {
        panic!("Expected Drop statement");
    };
    assert_eq!(drop.object_type, DropObjectType::Database);
    assert_eq!(drop.name.name.as_ref(), "test_db");
}
