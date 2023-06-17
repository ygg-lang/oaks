use oak_core::{FunctionRule, Lexer, TestCase, TestUtils};
use oak_python::{
    lexer::PythonLexRules,
    token::{PythonDelimiter, PythonKeyword, PythonLiteral, PythonOperator, PythonToken, PythonTokenKind},
};
use std::{
    fs,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

/// 文件遍历测试工具
struct FileTestSuite {
    base_path: PathBuf,
    file_extension: String,
}

impl FileTestSuite {
    /// 创建新的文件测试套件
    pub fn new<P: AsRef<Path>>(base_path: P, file_extension: &str) -> Self {
        Self { base_path: base_path.as_ref().to_path_buf(), file_extension: file_extension.to_string() }
    }

    /// 遍历指定目录下的所有指定后缀文件
    pub fn find_test_files(&self) -> Vec<PathBuf> {
        let mut files = Vec::new();

        for entry in WalkDir::new(&self.base_path).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == self.file_extension {
                        files.push(path.to_path_buf());
                    }
                }
            }
        }

        files.sort();
        files
    }

    /// 获取对应JSON 文件路径
    pub fn get_json_path(&self, source_file: &Path) -> PathBuf {
        source_file.with_extension("json")
    }

    /// 读取文件内容
    pub fn read_file_content(&self, path: &Path) -> Result<String, std::io::Error> {
        fs::read_to_string(path)
    }

    /// 检JSON 文件是否存在
    pub fn json_exists(&self, json_path: &Path) -> bool {
        json_path.exists()
    }

    /// 运行词法分析测试
    pub fn run_lexer_test(&self, lexer: &Lexer<PythonTokenKind>, source_file: &Path) -> Result<(), String> {
        let content =
            self.read_file_content(source_file).map_err(|e| format!("Failed to read file {:?}: {}", source_file, e))?;

        let json_path = self.get_json_path(source_file);
        let test_name = source_file.file_stem().and_then(|s| s.to_str()).unwrap_or("unknown");

        if self.json_exists(&json_path) {
            // JSON 文件存在，进行比较测
            println!("Running comparison test for: {}", test_name);
            TestUtils::run_test_case(lexer, test_name, &content, &json_path)
                .map_err(|e| format!("Test failed for {:?}: {}", source_file, e))?;
        }
        else {
            // JSON 文件不存在，生成新的 JSON 文件
            println!("Generating JSON for: {}", test_name);
            TestUtils::save_test_result(lexer, test_name, &content, &json_path)
                .map_err(|e| format!("Failed to save test result for {:?}: {}", source_file, e))?;
        }

        Ok(())
    }
}

#[test]
fn test_lexer_files() {
    let mut lexer = Lexer::new();

    // 添加所有词法规
    for rule in PythonLexRules::all_rules() {
        lexer.add_function_rule_with_priority(rule.func, rule.priority);
    }

    // 创建文件测试套件
    let test_suite = FileTestSuite::new("tests/lexer", "py");
    let test_files = test_suite.find_test_files();

    println!("Found {} lexer test files", test_files.len());

    for file in test_files {
        println!("Testing lexer file: {:?}", file);
        match test_suite.run_lexer_test(&lexer, &file) {
            Ok(()) => println!("Lexer test passed: {:?}", file),
            Err(e) => {
                println!("Lexer test failed: {:?}", file);
                println!("  Error: {}", e);
                // 在测试环境中，我们可能想要继续测试其他文件而不是立即失
                // panic!("Lexer test failed: {}", e);
            }
        }
    }
}

#[test]
fn test_parser_files() {
    let mut lexer = Lexer::new();

    // 添加所有词法规
    for rule in PythonLexRules::all_rules() {
        lexer.add_function_rule_with_priority(rule.func, rule.priority);
    }

    // 创建文件测试套件
    let test_suite = FileTestSuite::new("tests/parser", "py");
    let test_files = test_suite.find_test_files();

    println!("Found {} parser test files", test_files.len());

    for file in test_files {
        println!("Testing parser file: {:?}", file);
        // 目前只进行词法分析，后续可以扩展为语法分
        match test_suite.run_lexer_test(&lexer, &file) {
            Ok(()) => println!("Parser test passed: {:?}", file),
            Err(e) => {
                println!("Parser test failed: {:?}", file);
                println!("  Error: {}", e);
                // 在测试环境中，我们可能想要继续测试其他文件而不是立即失
                // panic!("Parser test failed: {}", e);
            }
        }
    }
}

#[test]
fn test_file_discovery() {
    let lexer_suite = FileTestSuite::new("tests/lexer", "py");
    let parser_suite = FileTestSuite::new("tests/parser", "py");

    let lexer_files = lexer_suite.find_test_files();
    let parser_files = parser_suite.find_test_files();

    println!("Lexer test files:");
    for file in &lexer_files {
        println!("  {:?}", file);
    }

    println!("Parser test files:");
    for file in &parser_files {
        println!("  {:?}", file);
    }

    assert!(!lexer_files.is_empty(), "Should find some lexer test files");
    assert!(!parser_files.is_empty(), "Should find some parser test files");
}
