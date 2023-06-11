use crate::{
    Language, Lexer, SyntaxKind,
    errors::{OakDiagnostics, OakError},
    helpers::{create_file, json_from_path, source_from_path},
};
use serde::{Deserialize, Serialize};
use serde_json::{Serializer, ser::PrettyFormatter};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct LexerTester {
    root: PathBuf,
    extensions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct LexerTestExpected {
    success: bool,
    count: usize,
    tokens: Vec<TokenData>,
    errors: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TokenData {
    kind: String,
    text: String,
    start: usize,
    end: usize,
}

impl LexerTester {
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        Self { root: root.as_ref().to_path_buf(), extensions: vec![] }
    }

    pub fn with_extension(mut self, extension: impl ToString) -> Self {
        self.extensions.push(extension.to_string());
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
        L: Language,
        L::SyntaxKind: Serialize + std::fmt::Debug,
        Lex: Lexer<L>,
    {
        let test_files = self.find_test_files()?;

        for file_path in test_files {
            println!("Testing file: {}", file_path.display());
            self.test_single_file(&file_path, &lexer)?;
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
        L: Language,
        L::SyntaxKind: Serialize + std::fmt::Debug,
        Lex: Lexer<L>,
    {
        let source = source_from_path(file_path)?;
        let OakDiagnostics { result, mut diagnostics } = lexer.lex(&source);

        // 获取tokens
        let tokens = match result {
            Ok(tokens) => tokens,
            Err(e) => {
                diagnostics.push(e);
                vec![]
            }
        };

        let tokens: Vec<TokenData> = tokens
            .into_iter()
            .filter(|token| !token.kind.is_trivia())
            .map(|token| {
                let text = source.get_text_in(token.span.clone().into()).unwrap_or_default().to_string();
                TokenData { kind: format!("{:?}", token.kind), text, start: token.span.start, end: token.span.end }
            })
            .take(100)
            .collect();

        let test_result = LexerTestExpected { success: true, count: tokens.len(), tokens, errors: vec![] };

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
