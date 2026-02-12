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
        panic!("Build error: {:?}\nDiagnostics: {:?}", e, result.diagnostics);
    }
    assert!(result.result.is_ok());
    let root = result.result.unwrap();
    assert_eq!(root.statements.len(), 1);

    if let SqlStatement::Select(select) = &root.statements[0] {
        assert_eq!(select.items.len(), 2);
        assert!(select.from.is_some());
        assert_eq!(select.from.as_ref().unwrap().name.name, "users");
    }
    else {
        panic!("Expected Select statement");
    }
}

#[test]
fn test_sql_builder_insert() {
    let config = SqlLanguage::default();
    let builder = SqlBuilder::new(&config);
    let source = SourceText::new("INSERT INTO users (name, age) VALUES ('John', 25);");

    let mut session = ParseSession::<SqlLanguage>::default();
    let result = builder.build(&source, &[], &mut session);

    if let Err(e) = &result.result {
        panic!("Build error: {:?}\nDiagnostics: {:?}", e, result.diagnostics);
    }
    assert!(result.result.is_ok());
    let root = result.result.unwrap();
    assert_eq!(root.statements.len(), 1);

    if let SqlStatement::Insert(insert) = &root.statements[0] {
        assert_eq!(insert.table_name.name.name, "users");
        assert_eq!(insert.columns.len(), 2);
        assert_eq!(insert.values.len(), 2);
    }
    else {
        panic!("Expected Insert statement");
    }
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

    if let SqlStatement::Create(create) = &root.statements[0] {
        assert_eq!(create.object_type, CreateObjectType::Table);
        assert_eq!(create.name.name, "users");
        if let CreateBody::Table { columns } = &create.body {
            assert_eq!(columns.len(), 2);
            assert_eq!(columns[0].name.name, "id");
            assert_eq!(columns[0].data_type, "INT");
            assert_eq!(columns[0].constraints.len(), 1);
            assert!(matches!(columns[0].constraints[0], ColumnConstraint::PrimaryKey { .. }));
            assert_eq!(columns[1].name.name, "name");
            assert_eq!(columns[1].data_type, "TEXT");
        }
        else {
            panic!("Expected Table body");
        }
    }
    else {
        panic!("Expected Create statement");
    }
}

#[test]
fn test_sql_builder_complex_create() {
    let config = SqlLanguage::default();
    let builder = SqlBuilder::new(&config);
    let source = SourceText::new("CREATE TABLE orders (id INT PRIMARY KEY AUTOINCREMENT, total DECIMAL(10,2) NOT NULL DEFAULT 0.0);");

    let mut session = ParseSession::<SqlLanguage>::default();
    let result = builder.build(&source, &[], &mut session);

    assert!(result.result.is_ok());
    let root = result.result.unwrap();

    if let SqlStatement::Create(create) = &root.statements[0] {
        assert_eq!(create.name.name, "orders");
        if let CreateBody::Table { columns, .. } = &create.body {
            assert_eq!(columns.len(), 2);

            // Column 1: id INT PRIMARY KEY AUTOINCREMENT
            let id_col = &columns[0];
            assert_eq!(id_col.name.name, "id");
            assert_eq!(id_col.data_type, "INT");
            assert_eq!(id_col.constraints.len(), 2);

            // Column 2: total DECIMAL(10,2) NOT NULL DEFAULT 0.0
            let total_col = &columns[1];
            assert_eq!(total_col.name.name, "total");
            // DECIMAL(10,2) - our parser currently captures the full type string
            assert_eq!(total_col.data_type, "DECIMAL(10,2)");
            assert_eq!(total_col.constraints.len(), 2);

            let has_default = total_col.constraints.iter().any(|c| matches!(c, ColumnConstraint::Default(..)));
            assert!(has_default);
        }
        else {
            panic!("Expected Table body");
        }
    }
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

    if let SqlStatement::Create(create) = &root.statements[0] {
        assert_eq!(create.object_type, CreateObjectType::View);
        assert_eq!(create.name.name, "active_users");
        if let CreateBody::View { query, .. } = &create.body {
            assert!(query.from.is_some());
            assert!(query.selection.is_some());
        }
        else {
            panic!("Expected View body");
        }
    }
}

#[test]
fn test_sql_builder_create_index() {
    let config = SqlLanguage::default();
    let builder = SqlBuilder::new(&config);
    let source = SourceText::new("CREATE UNIQUE INDEX idx_user_email ON users (email);");

    let mut session = ParseSession::<SqlLanguage>::default();
    let result = builder.build(&source, &[], &mut session);

    assert!(result.result.is_ok());
    let root = result.result.unwrap();

    if let SqlStatement::Create(create) = &root.statements[0] {
        assert_eq!(create.object_type, CreateObjectType::Index);
        assert_eq!(create.name.name, "idx_user_email");
        if let CreateBody::Index { table_name, columns, unique, .. } = &create.body {
            assert_eq!(table_name.name.name, "users");
            assert_eq!(columns.len(), 1);
            assert_eq!(columns[0].name, "email");
            assert!(*unique);
        }
        else {
            panic!("Expected Index body");
        }
    }
}

#[test]
fn test_sql_builder_update_delete() {
    let config = SqlLanguage::default();
    let builder = SqlBuilder::new(&config);

    // UPDATE
    let source_update = SourceText::new("UPDATE users SET name = 'John', age = 30 WHERE id = 1;");
    let mut session = ParseSession::<SqlLanguage>::default();
    let result_update = builder.build(&source_update, &[], &mut session);
    assert!(result_update.result.is_ok());
    let root_update = result_update.result.unwrap();
    if let SqlStatement::Update(update) = &root_update.statements[0] {
        assert_eq!(update.table_name.name.name, "users");
        assert_eq!(update.assignments.len(), 2);
        assert!(update.selection.is_some());
    }
    else {
        panic!("Expected Update statement");
    }

    // DELETE
    let source_delete = SourceText::new("DELETE FROM users WHERE id = 1;");
    let result_delete = builder.build(&source_delete, &[], &mut session);
    assert!(result_delete.result.is_ok());
    let root_delete = result_delete.result.unwrap();
    if let SqlStatement::Delete(delete) = &root_delete.statements[0] {
        assert_eq!(delete.table_name.name.name, "users");
        assert!(delete.selection.is_some());
    }
    else {
        panic!("Expected Delete statement");
    }
}

#[test]
fn test_sql_builder_drop_alter() {
    let config = SqlLanguage::default();
    let builder = SqlBuilder::new(&config);

    // DROP
    let source_drop = SourceText::new("DROP TABLE IF EXISTS users;");
    let mut session = ParseSession::<SqlLanguage>::default();
    let result_drop = builder.build(&source_drop, &[], &mut session);
    assert!(result_drop.result.is_ok());
    let root_drop = result_drop.result.unwrap();
    if let SqlStatement::Drop(drop) = &root_drop.statements[0] {
        assert_eq!(drop.object_type, DropObjectType::Table);
        assert_eq!(drop.name.name, "users");
        assert!(drop.if_exists);
    }
    else {
        panic!("Expected Drop statement");
    }

    // ALTER
    let source_alter = SourceText::new("ALTER TABLE users RENAME TO customers;");
    let result_alter = builder.build(&source_alter, &[], &mut session);
    assert!(result_alter.result.is_ok());
    let root_alter = result_alter.result.unwrap();
    if let SqlStatement::Alter(alter) = &root_alter.statements[0] {
        assert_eq!(alter.table_name.name.name, "users");
    }
    else {
        panic!("Expected Alter statement");
    }
}

#[test]
fn test_sql_builder_database_stmt() {
    let config = SqlLanguage::default();
    let builder = SqlBuilder::new(&config);
    let mut session = ParseSession::<SqlLanguage>::default();

    // CREATE DATABASE
    let source_create = SourceText::new("CREATE DATABASE test_db;");
    let result_create = builder.build(&source_create, &[], &mut session);
    assert!(result_create.result.is_ok());
    let root_create = result_create.result.unwrap();
    if let SqlStatement::Create(create) = &root_create.statements[0] {
        assert_eq!(create.object_type, CreateObjectType::Database);
        assert_eq!(create.name.name, "test_db");
        assert!(matches!(create.body, CreateBody::Database));
    }
    else {
        panic!("Expected Create statement");
    }

    // DROP DATABASE
    let source_drop = SourceText::new("DROP DATABASE test_db;");
    let result_drop = builder.build(&source_drop, &[], &mut session);
    assert!(result_drop.result.is_ok());
    let root_drop = result_drop.result.unwrap();
    if let SqlStatement::Drop(drop) = &root_drop.statements[0] {
        assert_eq!(drop.object_type, DropObjectType::Database);
        assert_eq!(drop.name.name, "test_db");
    }
    else {
        panic!("Expected Drop statement");
    }
}
