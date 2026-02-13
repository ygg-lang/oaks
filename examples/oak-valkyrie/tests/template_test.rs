use oak_core::{Builder, Parser, SourceText, parser::ParseSession};
use oak_valkyrie::{ValkyrieBuilder, ValkyrieLanguage, ValkyrieParser};
use oak_dejavu::language::SyntaxMode;

#[test]
fn test_template_parsing() {
    let mut language = ValkyrieLanguage::default();
    language.syntax_mode = SyntaxMode::Template;
    
    let parser = ValkyrieParser::new(&language);
    let builder = ValkyrieBuilder::new(&language);
    
    let source = SourceText::new("Hello, {name}! <% if true { %> Welcome <% } %>");
    let mut session = ParseSession::new(1024);
    
    let result = parser.parse(&source, &[], &mut session);
    assert!(!result.has_errors(), "Template should parse without errors: {:?}", result.diagnostics);
    
    let built = builder.build(&source, &[], &mut session);
    assert!(!built.has_errors(), "Template should build without errors: {:?}", built.diagnostics);
    
    let ast = built.result.unwrap();
    let ast_str = format!("{:?}", ast);
    
    assert!(ast_str.contains("TemplateText { content: \"Hello, \""), "Should contain 'Hello, '");
    assert!(ast_str.contains("TemplateInterpolation"), "Should contain interpolation");
    assert!(ast_str.contains("TemplateControl"), "Should contain control block");
    assert!(ast_str.contains("TemplateText { content: \" Welcome \""), "Should contain ' Welcome '");
}
