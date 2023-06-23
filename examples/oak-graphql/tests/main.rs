use oak_core::{ParseSession, SourceText, lexer::Lexer};
use oak_graphql::{kind::GraphQLSyntaxKind, language::GraphQLLanguage, lexer::GraphQLLexer};
use std::fs;

struct GraphQLFileTestSuite {
    test_dir: String,
    extension: String,
}

impl GraphQLFileTestSuite {
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

    fn _get_json_path(&self, file_path: &str) -> String {
        file_path.replace(&format!(".{}", self.extension), ".json")
    }

    fn read_file_content(&self, file_path: &str) -> Result<String, std::io::Error> {
        fs::read_to_string(file_path)
    }
}

fn test_lexer() {
    println!("Testing GraphQL Lexer...");

    let language = GraphQLLanguage {};
    let lexer = GraphQLLexer::new(&language);

    let content = "query { user { name } }";
    let source = SourceText::new(content);
    let mut session = ParseSession::<GraphQLLanguage>::default();
    let lex_result = lexer.lex(&source, &[], &mut session);

    match lex_result.result {
        Ok(tokens) => {
            println!("Lexed {} tokens", tokens.len());

            // 检查是否有 EOF token
            if let Some(last_token) = tokens.last() {
                if last_token.kind == GraphQLSyntaxKind::Eof {
                    println!("✓ Found EOF token");
                }
                else {
                    println!("✗ Missing EOF token");
                }
            }
        }
        Err(e) => {
            println!("✗ Lexer error: {}", e)
        }
    }

    if !lex_result.diagnostics.is_empty() {
        println!("Diagnostics: {} warnings/errors", lex_result.diagnostics.len())
    }
}

fn test_parser() {
    println!("Testing GraphQL Parser...");

    let test_suite = GraphQLFileTestSuite::new("tests/files", "graphql");
    let test_files = test_suite.find_test_files();

    for file_path in test_files {
        println!("Testing file: {}", file_path);

        match test_suite.read_file_content(&file_path) {
            Ok(content) => {
                // 首先进行词法分析
                let language = GraphQLLanguage {};
                let lexer = GraphQLLexer::new(&language);
                let source = SourceText::new(&content);
                let mut session = ParseSession::<GraphQLLanguage>::default();
                let lex_result = lexer.lex(&source, &[], &mut session);

                match lex_result.result {
                    Ok(tokens) => {
                        println!("  Lexing completed successfully");
                        println!("  Tokens found: {}", tokens.len());

                        // TODO: 添加 GraphQL 解析器实现
                        // let mut files = GraphQLParser::new(tokens);
                        // match files.parse() {
                        //     Ok(ast) => {
                        //         println!("  Parsing completed successfully");
                        //     }
                        //     Err(e) => {
                        //         println!("  Parse error: {:?}", e);
                        //     }
                        // }
                    }
                    Err(e) => {
                        println!("  Lexer error: {}", e)
                    }
                }

                if !lex_result.diagnostics.is_empty() {
                    println!("  Diagnostics: {} warnings/errors", lex_result.diagnostics.len())
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
    println!("Running GraphQL Language Tests");
    println!("==============================");

    test_lexer();
    test_parser();

    println!("All tests completed!")
}
