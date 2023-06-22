//! Lexer testing utilities for the Oak parsing framework.
//!
//! This module provides comprehensive testing infrastructure for lexers,
//! including file-based testing, expected output comparison, and
//! test result serialization.

use crate::{
    Language, Lexer, TokenType,
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
    pub fn run_tests<L, Lex>(self, lexer: &Lex) -> Result<(), OakError>
    where
        L: Language + Send + Sync + 'static,
        L::TokenType: Serialize + std::fmt::Debug + Send + Sync,
        Lex: Lexer<L> + Send + Sync + 'static + Clone,
    {
        let test_files = self.find_test_files()?;
        let force_regenerated = std::env::var("REGENERATE_TESTS").unwrap_or("0".to_string()) == "1";
        let mut regenerated_any = false;

        for file_path in test_files {
            println!("Testing file: {}", file_path.display());
            regenerated_any |= self.test_single_file::<L, Lex>(&file_path, lexer, force_regenerated)?;
        }

        if regenerated_any && force_regenerated { Err(OakError::test_regenerated(self.root)) } else { Ok(()) }
    }

    fn find_test_files(&self) -> Result<Vec<PathBuf>, OakError> {
        let mut files = Vec::new();

        for entry in WalkDir::new(&self.root) {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    let ext_str = ext.to_str().unwrap_or("");
                    if self.extensions.iter().any(|e| e == ext_str) {
                        // 忽略由 Tester 自身生成的输出文件，防止递归包含
                        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                        let is_output_file = file_name.ends_with(".parsed.json") || file_name.ends_with(".lexed.json") || file_name.ends_with(".built.json");

                        if !is_output_file {
                            files.push(path.to_path_buf());
                        }
                    }
                }
            }
        }

        Ok(files)
    }

    fn test_single_file<L, Lex>(&self, file_path: &Path, lexer: &Lex, force_regenerated: bool) -> Result<bool, OakError>
    where
        L: Language + Send + Sync + 'static,
        L::TokenType: Serialize + std::fmt::Debug + Send + Sync,
        Lex: Lexer<L> + Send + Sync + 'static + Clone,
    {
        let source = source_from_path(file_path)?;

        // Use Arc and Mutex to share results between threads
        let result = Arc::new(Mutex::new(None));
        let result_clone = Arc::clone(&result);

        // Clone lexer for use in thread
        let lexer_clone = lexer.clone();
        // Wrap source in Arc for sharing between threads
        let source_arc = Arc::new(source);
        let source_clone = Arc::clone(&source_arc);

        // Create a new thread to perform lexical analysis
        let handle = thread::spawn(move || {
            let mut cache = crate::parser::ParseSession::<L>::default();
            let output = lexer_clone.lex(&*source_clone, &[], &mut cache);
            let mut result = result_clone.lock().unwrap();
            *result = Some(output);
        });

        // Wait for thread completion or timeout
        let start_time = Instant::now();
        let timeout_occurred = loop {
            // Check if thread has finished
            if handle.is_finished() {
                break false;
            }

            // Check for timeout
            if start_time.elapsed() > self.timeout {
                break true;
            }

            // Sleep briefly to avoid busy waiting
            thread::sleep(Duration::from_millis(10));
        };

        // Return error if timed out
        if timeout_occurred {
            return Err(OakError::custom_error(&format!("Lexer test timed out after {:?} for file: {}", self.timeout, file_path.display())));
        }

        // Get lexical analysis result
        let OakDiagnostics { result: tokens_result, mut diagnostics } = {
            let result_guard = result.lock().unwrap();
            match result_guard.as_ref() {
                Some(output) => output.clone(),
                None => return Err(OakError::custom_error("Failed to get lexer result")),
            }
        };

        // Construct test result
        let mut success = true;
        let tokens = match tokens_result {
            Ok(tokens) => tokens,
            Err(e) => {
                success = false;
                diagnostics.push(e);
                triomphe::Arc::from_iter(Vec::new())
            }
        };

        if !diagnostics.is_empty() {
            success = false;
        }

        let tokens: Vec<TokenData> = tokens
            .iter()
            .filter(|token| !token.kind.is_ignored())
            .map(|token| {
                let len = source_arc.as_ref().length();
                let start = token.span.start.min(len);
                let end = token.span.end.min(len).max(start);
                let text = source_arc.as_ref().get_text_in((start..end).into()).to_string();
                TokenData { kind: format!("{:?}", token.kind), text, start: token.span.start, end: token.span.end }
            })
            .take(100)
            .collect();

        let errors: Vec<String> = diagnostics.iter().map(|e| e.to_string()).collect();
        let test_result = LexerTestExpected { success, count: tokens.len(), tokens, errors };

        // Process expected result file
        let expected_file = file_path.with_extension(format!("{}.lexed.json", file_path.extension().unwrap_or_default().to_str().unwrap_or("")));

        let mut regenerated = false;
        if expected_file.exists() && !force_regenerated {
            let expected: LexerTestExpected = json_from_path(&expected_file)?;

            if test_result != expected {
                return Err(OakError::test_failure(file_path.to_path_buf(), format!("{:#?}", expected), format!("{:#?}", test_result)));
            }
        }
        else {
            let file = create_file(&expected_file)?;
            let mut writer = Serializer::with_formatter(file, PrettyFormatter::with_indent(b"    "));
            test_result.serialize(&mut writer)?;

            if force_regenerated {
                regenerated = true;
            }
            else {
                return Err(OakError::test_regenerated(expected_file));
            }
        }

        Ok(regenerated)
    }
}
