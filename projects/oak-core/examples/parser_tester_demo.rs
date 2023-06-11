use oak_core::{
    GreenLeaf, GreenNode, GreenTree, Language, Parser, SourceText, SyntaxKind,
    helpers::parsing::{ParserTester, run_parser_tests},
    parser::ParseOutput,
};
use serde::Serialize;
use std::rc::Rc;

// 示例语言定义
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DemoSyntaxKind {
    Root,
    Number,
    Plus,
    Minus,
    Expression,
    Whitespace,
    Error,
}

impl SyntaxKind for DemoSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace)
    }

    fn is_comment(&self) -> bool {
        false
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace)
    }

    fn is_token_type(&self) -> bool {
        matches!(self, Self::Number | Self::Plus | Self::Minus | Self::Whitespace)
    }

    fn is_element_type(&self) -> bool {
        matches!(self, Self::Root | Self::Expression)
    }
}

#[derive(Debug, Clone)]
pub struct DemoLanguage;

impl Language for DemoLanguage {
    type SyntaxKind = DemoSyntaxKind;
}

// 简单的示例解析器
pub struct DemoParser;

impl Parser<DemoLanguage> for DemoParser {
    fn parse(&self, source: &SourceText) -> ParseOutput<DemoSyntaxKind> {
        // 这是一个非常简单的示例解析器，只是为了演示 ParserTester
        use oak_core::errors::OakDiagnostics;

        let text = source.get_text_in((0..source.len()).into()).unwrap_or("");

        // 创建一个简单的 AST：Root -> Expression -> Number
        let number_leaf = GreenLeaf::new(DemoSyntaxKind::Number, text.len());
        let expression_children = vec![GreenTree::Leaf(number_leaf)];
        let expression_node = GreenNode::new(DemoSyntaxKind::Expression, expression_children);

        let root_children = vec![GreenTree::Node(expression_node)];
        let root_node = GreenNode::new(DemoSyntaxKind::Root, root_children);

        OakDiagnostics { result: Ok(root_node), diagnostics: vec![] }
    }

    fn parse_tokens(&self, _source: &SourceText, _tokens: &[oak_core::Token<DemoSyntaxKind>]) -> ParseOutput<DemoSyntaxKind> {
        unimplemented!("parse_tokens not implemented for demo")
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("ParserTester Demo");
    println!("================");

    // 创建测试目录和文件
    std::fs::create_dir_all("test_data")?;
    std::fs::write("test_data/simple.demo", "42")?;
    std::fs::write("test_data/expression.demo", "1 + 2")?;

    // 创建解析器
    let parser = DemoParser;

    // 运行测试
    println!("Running parser tests...");

    let tester = ParserTester::new("test_data").with_extension("demo");

    match tester.run_tests(parser) {
        Ok(()) => println!("All tests passed!"),
        Err(e) => println!("Test failed: {:?}", e),
    }

    // 清理测试文件
    std::fs::remove_dir_all("test_data")?;

    Ok(())
}
