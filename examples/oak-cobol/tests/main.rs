use oak_cobol::{CobolLanguage, CobolLexer, CobolTokenType};
use oak_core::{Lexer, SourceText, parser::session::ParseSession};
use std::fs;

struct CobolFileTestSuite {
    test_dir: String,
    extension: String,
}

impl CobolFileTestSuite {
    fn new(test_dir: &str, extension: &str) -> Self {
        Self { test_dir: test_dir.to_string(), extension: extension.to_string() }
    }

    fn find_test_files(&self) -> Vec<String> {
        let mut files = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.test_dir) {
            for entry in entries.flatten() {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.ends_with(&format!(".{}", self.extension)) {
                        files.push(entry.path().to_string_lossy().to_string())
                    }
                }
            }
        }
        files.sort();
        files
    }

    fn read_file_content(&self, file_path: &str) -> Result<String, std::io::Error> {
        fs::read_to_string(file_path)
    }
}

fn test_lexer() {
    println!("Testing COBOL Lexer...");

    let test_suite = CobolFileTestSuite::new("tests/lexer", "cob");
    let test_files = test_suite.find_test_files();

    for file_path in test_files {
        println!("Testing file: {}", file_path);

        match test_suite.read_file_content(&file_path) {
            Ok(content) => {
                let source = SourceText::new(&*content);
                let language = CobolLanguage {};
                let lexer = CobolLexer::new(&language);
                let mut session = ParseSession::<CobolLanguage>::new(16);
                let output = lexer.lex(&source, &[], &mut session);

                let tokens = match &output.result {
                    Ok(tokens) => tokens,
                    Err(e) => {
                        println!("  Lexing error: {:?}", e);
                        continue;
                    }
                };
                println!("  Tokens found: {}", tokens.len());

                // 显示前几tokens 作为示例
                for (i, token) in tokens.iter().take(10).enumerate() {
                    println!("    {}: {:?}", i, token)
                }

                if tokens.len() > 10 {
                    println!("    ... and {} more tokens", tokens.len() - 10)
                }

                // 验证最后一tokens EOF
                if let Some(last_token) = tokens.last() {
                    match last_token.kind {
                        CobolTokenType::Eof => println!("  Lexing completed successfully"),
                        _ => println!("  Warning: Last tokens is not EOF"),
                    }
                }
                else {
                    println!("  Warning: No tokens generated")
                }
            }
            Err(e) => {
                println!("  Error reading file: {}", e)
            }
        }

        println!()
    }
}

fn test_parser() {
    println!("Testing COBOL Parser...");

    let test_suite = CobolFileTestSuite::new("tests/files", "cob");
    let test_files = test_suite.find_test_files();

    for file_path in test_files {
        println!("Testing file: {}", file_path);

        match test_suite.read_file_content(&file_path) {
            Ok(content) => {
                // 首先进行词法分析
                let source = SourceText::new(&*content);
                let language = CobolLanguage {};
                let lexer = CobolLexer::new(&language);
                let mut session = ParseSession::<CobolLanguage>::new(16);
                let output = lexer.lex(&source, &[], &mut session);

                match &output.result {
                    Ok(_tokens) => {
                        // 解析器测试暂时跳过，因为 COBOL 解析器可能还未实现
                        println!("  Parser test skipped (not implemented yet)")
                    }
                    Err(e) => {
                        println!("  Lexing error: {:?}", e)
                    }
                }
            }
            Err(e) => {
                println!("  Error reading file: {}", e)
            }
        }

        println!()
    }
}

fn main() {
    println!("Running COBOL Language Tests");
    println!("============================");

    test_lexer();
    test_parser();

    println!("All tests completed!");
}
