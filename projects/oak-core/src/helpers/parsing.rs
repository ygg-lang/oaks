//! Parser testing utilities for the Oak parsing framework.
//!
//! This module provides comprehensive testing infrastructure for parsers,
//! including file-based testing, expected output comparison, timeout handling,
//! and test result serialization.

use crate::{
    Language, Parser,
    errors::OakError,
    helpers::{create_file, json_from_path, source_from_path},
};
use serde::{Deserialize, Serialize};
use serde_json::{Serializer, ser::PrettyFormatter};
use std::{
    fmt::Debug,
    path::{Path, PathBuf},
    time::Duration,
};
use walkdir::WalkDir;

/// A concurrent parser testing utility that can run tests against multiple files with timeout support.
///
/// The `ParserTester` provides functionality to test parsers against a directory
/// of files with specific extensions, comparing actual output against expected
/// results stored in JSON files, with configurable timeout protection.
pub struct ParserTester {
    root: PathBuf,
    extensions: Vec<String>,
    timeout: Duration,
}

/// Expected parser test results for comparison.
///
/// This struct represents the expected output of a parser test, including
/// success status, node count, AST structure, and any expected errors.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ParserTestExpected {
    success: bool,
    node_count: usize,
    ast_structure: AstNodeData,
    errors: Vec<String>,
}

/// AST node data structure for parser testing.
///
/// Represents a node in the abstract kind tree with its kind, children,
/// text length, and leaf status used for testing parser output.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AstNodeData {
    kind: String,
    children: Vec<AstNodeData>,
    text_length: usize,
    is_leaf: bool,
}

impl ParserTester {
    /// Creates a new parser tester with the specified root directory and default 10-second timeout.
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        Self { root: root.as_ref().to_path_buf(), extensions: vec![], timeout: Duration::from_secs(10) }
    }

    /// Adds a file extension to test against.
    pub fn with_extension(mut self, extension: impl ToString) -> Self {
        self.extensions.push(extension.to_string());
        self
    }

    /// Sets the timeout for parsing operations.
    ///
    /// # Arguments
    ///
    /// * `timeout` - The maximum duration to wait for parsing to complete
    ///
    /// # Returns
    ///
    /// A new `ParserTester` with the specified timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
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
    pub fn run_tests<L, P>(self, parser: &P) -> Result<(), OakError>
    where
        P: Parser<L> + Send + Sync,
        L: Language + Send + Sync,
        L::SyntaxKind: Serialize + Debug + Sync + Send,
    {
        let test_files = self.find_test_files()?;

        for file_path in test_files {
            println!("Testing file: {}", file_path.display());
            self.test_single_file::<L, P>(&file_path, parser)?;
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
        P: Parser<L> + Send + Sync,
        L: Language + Send + Sync,
        L::SyntaxKind: Serialize + Debug + Sync + Send,
    {
        let source = source_from_path(file_path)?;

        // 在线程中执行解析并构造测试结果，主线程做真实超时控制
        use std::sync::mpsc;
        let (tx, rx) = mpsc::channel();
        let timeout = self.timeout;
        let file_path_string = file_path.display().to_string();

        std::thread::scope(|s| {
            s.spawn(move || {
                let parse_out = parser.parse(&source);

                // Build AST structure if parse succeeded, else create a minimal error node
                let (success, ast_structure) = match &parse_out.result {
                    Ok(root) => {
                        let ast = Self::to_ast::<L::SyntaxKind>(root);
                        (true, ast)
                    }
                    Err(_) => {
                        let ast = AstNodeData { kind: "Error".to_string(), children: vec![], text_length: 0, is_leaf: true };
                        (false, ast)
                    }
                };

                // Collect error messages
                let mut error_messages: Vec<String> = parse_out.diagnostics.iter().map(|e| e.to_string()).collect();
                if let Err(e) = &parse_out.result {
                    error_messages.push(e.to_string());
                }

                // Count nodes (including leaves)
                let node_count = Self::count_nodes(&ast_structure);

                let test_result = ParserTestExpected { success, node_count, ast_structure, errors: error_messages };

                let _ = tx.send(Ok::<ParserTestExpected, OakError>(test_result));
            });

            match rx.recv_timeout(timeout) {
                Ok(Ok(test_result)) => {
                    let expected_file = file_path.with_extension(format!(
                        "{}.expected.json",
                        file_path.extension().unwrap_or_default().to_str().unwrap_or("")
                    ));

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
                Ok(Err(err)) => Err(err),
                Err(mpsc::RecvTimeoutError::Timeout) => Err(OakError::custom_error(&format!(
                    "Parser test timed out after {:?} for file: {}",
                    timeout, file_path_string
                ))),
                Err(mpsc::RecvTimeoutError::Disconnected) => Err(OakError::custom_error(&format!(
                    "Parser test thread panicked or disconnected for file: {}",
                    file_path_string
                ))),
            }
        })
    }

    fn to_ast<K: Copy + Debug + Serialize>(root: &triomphe::Arc<crate::GreenNode<K>>) -> AstNodeData {
        let kind_str = format!("{:?}", root.kind);
        let children = root
            .children
            .iter()
            .map(|c| match c {
                crate::GreenTree::Node(n) => Self::to_ast(n),
                crate::GreenTree::Leaf(l) => {
                    AstNodeData { kind: format!("{:?}", l.kind), children: vec![], text_length: l.length, is_leaf: true }
                }
            })
            .collect::<Vec<_>>();
        AstNodeData { kind: kind_str, children, text_length: root.length, is_leaf: false }
    }

    fn count_nodes(node: &AstNodeData) -> usize {
        1 + node.children.iter().map(Self::count_nodes).sum::<usize>()
    }
}
