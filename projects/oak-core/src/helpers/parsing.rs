use crate::{
    Language, Parser, SyntaxKind,
    errors::{OakDiagnostics, OakError},
    helpers::{create_file, json_from_path, source_from_path},
    tree::{GreenNode, GreenTree},
};
use alloc::rc::Rc;
use serde::{Deserialize, Serialize};
use serde_json::{Serializer, ser::PrettyFormatter};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct ParserTester {
    root: PathBuf,
    extensions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ParserTestExpected {
    success: bool,
    node_count: usize,
    ast_structure: AstNodeData,
    errors: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AstNodeData {
    kind: String,
    children: Vec<AstNodeData>,
    text_length: usize,
    is_leaf: bool,
}

impl ParserTester {
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        Self { root: root.as_ref().to_path_buf(), extensions: vec![] }
    }

    pub fn with_extension(mut self, extension: impl ToString) -> Self {
        self.extensions.push(extension.to_string());
        self
    }

    /// Run tests for the given parser against all files in the root directory with the specified extensions.
    ///
    /// # Arguments
    ///
    /// * `parser`: The parser to test.
    ///
    /// # Examples
    ///
    /// ```
    /// use oak_core::helpers::parsing::ParserTester;
    ///
    /// let tester = ParserTester::new("tests/parser").with_extension("tex");
    /// tester.run_tests(my_parser)?;
    /// ```
    pub fn run_tests<L, P>(self, parser: P) -> Result<(), OakError>
    where
        L: Language,
        L::SyntaxKind: Serialize + std::fmt::Debug,
        P: Parser<L>,
    {
        let test_files = self.find_test_files()?;

        for file_path in test_files {
            println!("Testing file: {}", file_path.display());
            self.test_single_file(&file_path, &parser)?;
        }

        Ok(())
    }

    fn find_test_files(&self) -> Result<Vec<PathBuf>, OakError> {
        let mut files = Vec::new();

        for entry in WalkDir::new(&self.root) {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if self.extensions.iter().any(|e| e == ext.to_str().unwrap_or("")) {
                        files.push(path.to_path_buf());
                    }
                }
            }
        }

        Ok(files)
    }

    fn test_single_file<L, P>(&self, file_path: &Path, parser: &P) -> Result<(), OakError>
    where
        L: Language,
        L::SyntaxKind: Serialize + std::fmt::Debug,
        P: Parser<L>,
    {
        let source = source_from_path(file_path)?;
        let OakDiagnostics { result, diagnostics } = parser.parse(&source);

        // 获取 AST 根节点
        let (success, ast_structure, node_count) = match result {
            Ok(green_node) => {
                let ast_data = self.convert_green_node_to_data(&green_node);
                let count = self.count_nodes(&green_node);
                (true, ast_data, count)
            }
            Err(_) => {
                // 解析失败时创建一个空的 AST 结构
                let empty_ast = AstNodeData { kind: "ERROR".to_string(), children: vec![], text_length: 0, is_leaf: true };
                (false, empty_ast, 0)
            }
        };

        let error_messages: Vec<String> = diagnostics.into_iter().map(|err| format!("{:?}", err)).collect();

        let test_result = ParserTestExpected { success, node_count, ast_structure, errors: error_messages };

        let expected_file = file_path
            .with_extension(format!("{}.expected.json", file_path.extension().unwrap_or_default().to_str().unwrap_or("")));

        let force_regenerated = std::env::var("REGENERATE_TESTS").unwrap_or("0".to_string()) == "1";

        if expected_file.exists() && !force_regenerated {
            let expected: ParserTestExpected = json_from_path(&expected_file)?;

            if test_result != expected {
                println!("Test failed for file: {}", file_path.display());
                println!("Expected: {:#?}", expected);
                println!("Actual: {:#?}", test_result);
                return Err(OakError::custom_error("Test results do not match expected results"));
            }
        }
        else {
            let file = create_file(&expected_file)?;
            let mut writer = Serializer::with_formatter(file, PrettyFormatter::with_indent(b"    "));
            test_result.serialize(&mut writer)?;

            println!("Created expected result file: {}\nNeed rerun", expected_file.display());
        }

        Ok(())
    }

    fn convert_green_node_to_data<K: Copy + std::fmt::Debug>(&self, green_node: &Rc<GreenNode<K>>) -> AstNodeData {
        let children: Vec<AstNodeData> =
            green_node.children.iter().map(|child| self.convert_green_tree_to_data(child)).collect();

        AstNodeData {
            kind: format!("{:?}", green_node.kind),
            children,
            text_length: green_node.length,
            is_leaf: green_node.children.is_empty(),
        }
    }

    fn convert_green_tree_to_data<K: Copy + std::fmt::Debug>(&self, green_tree: &GreenTree<K>) -> AstNodeData {
        match green_tree {
            GreenTree::Node(node) => self.convert_green_node_to_data(node),
            GreenTree::Leaf(leaf) => {
                AstNodeData { kind: format!("{:?}", leaf.kind), children: vec![], text_length: leaf.length, is_leaf: true }
            }
        }
    }

    fn count_nodes<K: Copy>(&self, green_node: &Rc<GreenNode<K>>) -> usize {
        let mut count = 1; // 当前节点
        for child in &green_node.children {
            count += self.count_green_tree_nodes(child);
        }
        count
    }

    fn count_green_tree_nodes<K: Copy>(&self, green_tree: &GreenTree<K>) -> usize {
        match green_tree {
            GreenTree::Node(node) => self.count_nodes(node),
            GreenTree::Leaf(_) => 1,
        }
    }
}

/// 便利函数：为指定的解析器和文件扩展名运行解析器测试
///
/// # Arguments
///
/// * `test_dir` - 测试文件目录
/// * `extension` - 文件扩展名
/// * `parser` - 要测试的解析器
///
/// # Examples
///
/// ```
/// use oak_core::helpers::parsing::run_parser_tests;
///
/// run_parser_tests("tests/tex", "tex", my_tex_parser)?;
/// ```
pub fn run_parser_tests<L, P>(test_dir: impl AsRef<Path>, extension: &str, parser: P) -> Result<(), OakError>
where
    L: Language,
    L::SyntaxKind: Serialize + std::fmt::Debug,
    P: Parser<L>,
{
    ParserTester::new(test_dir).with_extension(extension).run_tests(parser)
}
