use alloc::string::ToString;
use oak_core::{FileTestSuite, Lexer, Parser, SourceSpan};
use oak_graphql::{CLanguage, CLexRules, CParser, CToken, CTokenKind};
use std::{fs, path::Path};

struct CFileTestSuite {
    test_dir: String,
    extension: String,
}

impl CFileTestSuite {
    fn new(test_dir: &str, extension: &str) -> Self {
        Self { test_dir: test_dir.to_string(), extension: extension.to_string() }
    }

    fn find_test_files(&self) -> Vec<String> {
        let mut files = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.test_dir) {
            for entry in entries.flatten() {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.ends_with(&format!(".{}", self.extension)) {
                        files.push(entry.path().to_string_lossy().to_string());
                    }
                }
            }
        }
        files.sort();
        files
    }

    fn get_json_path(&self, file_path: &str) -> String {
        file_path.replace(&format!(".{}", self.extension), ".json")
    }

    fn read_file_content(&self, file_path: &str) -> Result<String, std::io::Error> {
        fs::read_to_string(file_path)
    }
}

fn test_lexer() {
    println!("Testing C Lexer...");

    let test_suite = CFileTestSuite::new("tests/lexer", "c");
    let test_files = test_suite.find_test_files();

    for file_path in test_files {
        println!("Testing file: {}", file_path);

        match test_suite.read_file_content(&file_path) {
            Ok(content) => {
                let mut lexer = Lexer::new(&content, CLexRules);
                let tokens = lexer.tokenize();

                println!("  Tokens found: {}", tokens.len());

                // 显示前几tokens 作为示例
                for (i, token) in tokens.iter().take(10).enumerate() {
                    println!("    {}: {:?}", i, token);
                }

                if tokens.len() > 10 {
                    println!("    ... and {} more tokens", tokens.len() - 10);
                }

                // 验证最后一tokens EOF
                if let Some(last_token) = tokens.last() {
                    match last_token.kind {
                        CTokenKind::Eof => println!("  Lexing completed successfully"),
                        _ => println!("  Warning: Last tokens is not EOF"),
                    }
                }
                else {
                    println!("  Warning: No tokens generated");
                }
            }
            Err(e) => {
                println!("  Error reading file: {}", e);
            }
        }

        println!();
    }
}

fn test_parser() {
    println!("Testing C Parser...");

    let test_suite = CFileTestSuite::new("tests/parser", "c");
    let test_files = test_suite.find_test_files();

    for file_path in test_files {
        println!("Testing file: {}", file_path);

        match test_suite.read_file_content(&file_path) {
            Ok(content) => {
                // 首先进行词法分析
                let mut lexer = Lexer::new(&content, CLexRules);
                let tokens = lexer.tokenize();

                // 然后进行语法分析
                let mut parser = CParser::new(tokens);
                match parser.parse() {
                    Ok(ast) => {
                        println!("  Parsing completed successfully");
                        println!(
                            "  AST root: TranslationUnit with {} external declarations",
                            ast.translation_unit.external_declarations.len()
                        );
                    }
                    Err(e) => {
                        println!("  Parse error: {:?}", e);
                    }
                }
            }
            Err(e) => {
                println!("  Error reading file: {}", e);
            }
        }

        println!();
    }
}

fn main() {
    println!("Running C Language Tests");
    println!("========================");

    test_lexer();
    test_parser();

    println!("All tests completed!");
}
