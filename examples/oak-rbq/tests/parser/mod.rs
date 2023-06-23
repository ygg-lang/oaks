use oak_core::{ParseSession, Parser, SourceText};
use oak_rbq::{RbqLanguage, RbqParser, RbqSyntaxKind};

#[test]
fn test_rbq_parser_basic() {
    let source = SourceText::new("@schema(name=\"test\") namespace App { struct User { id: i32 } }");
    let language = RbqLanguage::default();
    let parser = RbqParser::new(&language);

    let mut session = ParseSession::<RbqLanguage>::new(16);
    let result = parser.parse(&source, &[], &mut session);

    assert!(result.diagnostics.is_empty(), "解析应该没有错误: {:?}", result.diagnostics)
}

#[test]
fn test_rbq_parser_complex() {
    let source = SourceText::new(
        r#"
        @schema(name="test")
        namespace App {
            @primary_key
            struct User {
                id: i32
                name: String
                role: Role
                tags: List<String>?
            }

            enum Role {
                Admin,
                User,
                Guest,
            }
        }
    "#,
    );
    let language = RbqLanguage::default();
    let parser = RbqParser::new(&language);

    let mut session = ParseSession::<RbqLanguage>::new(16);
    let result = parser.parse(&source, &[], &mut session);

    assert!(result.diagnostics.is_empty(), "解析应该没有错误: {:?}", result.diagnostics)
}
