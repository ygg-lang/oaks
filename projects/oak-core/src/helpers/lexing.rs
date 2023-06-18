//! Lexer testing utilities for the Oak parsing framework.
//!
//! This module provides comprehensive testing infrastructure for lexers,
//! including file-based testing, expected output comparison, and
//! test result serialization.

use crate::{
    Language, Lexer, SyntaxKind,
    errors::{OakDiagnostics, OakError},
    helpers::{create_file, json_from_path, source_from_path},
    source::Source,
};
use serde::{Deserialize, Serialize};
use serde_json::{Serializer, ser::PrettyFormatter};
use std::{
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};
use walkdir::WalkDir;

/// A lexer testing utility that can run tests against multiple files.
///
/// The `LexerTester` provides functionality to test lexers against a directory
/// of files with specific extensions, comparing actual output against expected
/// results stored in JSON files.
pub struct LexerTester {
    root: PathBuf,
    extensions: Vec<String>,
    timeout: Duration,
}

/// Expected lexer test results for comparison.
///
/// This struct represents the expected output of a lexer test, including
/// success status, token count, token data, and any expected errors.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct LexerTestExpected {
    success: bool,
    count: usize,
    tokens: Vec<TokenData>,
    errors: Vec<String>,
}

/// Individual token data for lexer testing.
///
/// Represents a single token with its kind, text content, and position
/// information used for testing lexer output.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TokenData {
    kind: String,
    text: String,
    start: usize,
    end: usize,
}

impl LexerTester {
    /// Creates a new lexer tester with the specified root directory.
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        Self { root: root.as_ref().to_path_buf(), extensions: vec![], timeout: Duration::from_secs(10) }
    }

    /// Adds a file extension to test against.
    pub fn with_extension(mut self, extension: impl ToString) -> Self {
        self.extensions.push(extension.to_string());
        self
    }
    /// Sets the timeout duration for each test.
    pub fn with_timeout(mut self, time: Duration) -> Self {
        self.timeout = time;
        self
    }

    /// Run tests for the given lexer against all files in the root directory with the specified extensions.
    ///
    /// # Arguments
    ///
    /// * `lexer`: The lexer to test.
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub fn run_tests<L, Lex>(self, lexer: Lex) -> Result<(), OakError>
    where
        L: Language + Send + Sync + 'static,
        L::SyntaxKind: Serialize + std::fmt::Debug + Send + Sync,
        Lex: Lexer<L> + Send + Sync + 'static + Clone,
    {
        let test_files = self.find_test_files()?;

        for file_path in test_files {
            println!("Testing file: {}", file_path.display());
            self.test_single_file::<L, Lex>(&file_path, &lexer)?;
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

    fn test_single_file<L, Lex>(&self, file_path: &Path, lexer: &Lex) -> Result<(), OakError>
    where
        L: Language + Send + Sync + 'static,
        L::SyntaxKind: Serialize + std::fmt::Debug + Send + Sync,
        Lex: Lexer<L> + Send + Sync + 'static + Clone,
    {
        let source = source_from_path(file_path)?;

        // 使用Arc和Mutex来在线程间共享结果
        let result = Arc::new(Mutex::new(None));
        let result_clone = Arc::clone(&result);

        // 克隆lexer以便在线程中使用
        let lexer_clone = lexer.clone();
        // 将source包装在Arc中以便在线程间共享
        let source_arc = Arc::new(source);
        let source_clone = Arc::clone(&source_arc);

        // 创建一个新线程来执行词法分析
        let handle = thread::spawn(move || {
            let mut builder = crate::GreenBuilder::new(0);
            let cache = crate::IncrementalCache::new(&mut builder);
            let output = lexer_clone.lex_incremental(&*source_clone, 0, cache);
            let mut result = result_clone.lock().unwrap();
            *result = Some(output);
        });

        // 等待线程完成或超时
        let start_time = Instant::now();
        let timeout_occurred = loop {
            // 检查线程是否已完成
            if handle.is_finished() {
                break false;
            }

            // 检查是否超时
            if start_time.elapsed() > self.timeout {
                break true;
            }

            // 短暂休眠以避免忙等待
            thread::sleep(Duration::from_millis(10));
        };

        // 如果超时，返回错误
        if timeout_occurred {
            return Err(OakError::custom_error(&format!(
                "Lexer test timed out after {:?} for file: {}",
                self.timeout,
                file_path.display()
            )));
        }

        // 获取词法分析结果
        let OakDiagnostics { result: tokens_result, mut diagnostics } = {
            let result_guard = result.lock().unwrap();
            match result_guard.as_ref() {
                Some(output) => output.clone(),
                None => return Err(OakError::custom_error("Failed to get lexer result")),
            }
        };

        // 构造测试结果
        let mut success = true;
        let tokens = match tokens_result {
            Ok(tokens) => tokens,
            Err(e) => {
                success = false;
                diagnostics.push(e);
                Vec::new()
            }
        };

        if !diagnostics.is_empty() {
            success = false;
        }

        let tokens: Vec<TokenData> = tokens
            .into_iter()
            .filter(|token| !token.kind.is_trivia())
            .map(|token| {
                let text = source_arc.as_ref().get_text_in(token.span.clone().into()).to_string();
                TokenData { kind: format!("{:?}", token.kind), text, start: token.span.start, end: token.span.end }
            })
            .take(100)
            .collect();

        let errors: Vec<String> = diagnostics.iter().map(|e| e.to_string()).collect();
        let test_result = LexerTestExpected { success, count: tokens.len(), tokens, errors };

        // 处理预期结果文件
        let expected_file = file_path
            .with_extension(format!("{}.expected.json", file_path.extension().unwrap_or_default().to_str().unwrap_or("")));

        let force_regenerated = std::env::var("REGENERATE_TESTS").unwrap_or("0".to_string()) == "1";

        if expected_file.exists() && !force_regenerated {
            let expected: LexerTestExpected = json_from_path(&expected_file)?;

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
}
