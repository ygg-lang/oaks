#![doc = include_str!("readme.md")]

use crate::{ast::*, kind::WolframSyntaxKind};
use alloc::{boxed::Box, string::String, vec::Vec};
use oak_core::{
    SourceLocation, SourceText, Token,
    errors::{OakDiagnostics, OakError},
};

type WolframToken = Token<WolframSyntaxKind>;

/// Wolfram 解析
#[derive(Debug)]
pub struct WolframParser {
    _source: Option<SourceText>,
    tokens: Vec<WolframToken>,
    position: usize,
    errors: Vec<OakError>,
}

impl WolframParser {
    /// 创建新的 WAT 解析
    pub fn new() -> Self {
        Self { tokens: Vec::new(), position: 0, errors: Vec::new(), _source: None }
    }

    /// 解析 WAT 代码
    pub fn parse(&mut self, tokens: Vec<WolframToken>) -> OakDiagnostics<WolframRoot> {
        self.tokens = tokens;
        self.position = 0;
        self.errors.clear();

        let mut items = Vec::new();

        while !self.is_at_end() {
            match self.parse_item() {
                Ok(item) => items.push(item),
                Err(err) => {
                    self.errors.push(err);
                    self.advance();
                }
            }
        }

        let diagnostics = core::mem::take(&mut self.errors);
        OakDiagnostics { result: Ok(WolframRoot { items }), diagnostics }
    }

    /// 解析顶级项目
    fn parse_item(&mut self) -> Result<WolframItem, OakError> {
        if self.match_token(&WolframSyntaxKind::LeftParen) {
            if self.match_token(&WolframSyntaxKind::Module) {
                return Ok(WolframItem::Module(self.parse_module()?));
            }
            else if self.match_token(&WolframSyntaxKind::Function) {
                return Ok(WolframItem::Function(self.parse_function()?));
            }
            else if self.match_token(&WolframSyntaxKind::Module) {
                return Ok(WolframItem::Memory(self.parse_memory()?));
            }
            else if self.match_token(&WolframSyntaxKind::Export) {
                return Ok(WolframItem::Export(self.parse_export()?));
            }
            else if self.match_token(&WolframSyntaxKind::Import) {
                return Ok(WolframItem::Import(self.parse_import()?));
            }
        }

        Err(self.make_error("Expected item"))
    }

    /// 解析模块
    fn parse_module(&mut self) -> Result<WolframModule, OakError> {
        let name = if self.check(&WolframSyntaxKind::Identifier) {
            self.advance();
            Some(String::new())
        }
        else {
            None
        };

        let mut items = Vec::new();

        while !self.check(&WolframSyntaxKind::RightParen) && !self.is_at_end() {
            match self.parse_item() {
                Ok(item) => items.push(item),
                Err(err) => {
                    self.errors.push(err);
                    self.advance();
                }
            }
        }

        self.consume(&WolframSyntaxKind::RightParen, "Expected ')' ")?;

        Ok(WolframModule { name, items })
    }

    /// 解析函数
    fn parse_function(&mut self) -> Result<WolframFunction, OakError> {
        let name = if self.check(&WolframSyntaxKind::Identifier) {
            self.advance();
            Some(String::new())
        }
        else {
            None
        };

        let params = Vec::new();
        let result = None;
        let locals = Vec::new();
        let body = Vec::new();

        self.consume(&WolframSyntaxKind::RightParen, "Expected ')' ")?;

        Ok(WolframFunction { name, params, result, locals, body })
    }

    /// 解析内存
    fn parse_memory(&mut self) -> Result<WolframMemory, OakError> {
        let name = if self.check(&WolframSyntaxKind::Identifier) {
            self.advance();
            Some(String::new())
        }
        else {
            None
        };

        let min = 1;
        let max = None;

        self.consume(&WolframSyntaxKind::RightParen, "Expected ')' ")?;

        Ok(WolframMemory { name, min, max })
    }

    /// 解析导出
    fn parse_export(&mut self) -> Result<WolframExport, OakError> {
        let name = if self.check(&WolframSyntaxKind::String) {
            self.advance();
            String::new()
        }
        else {
            String::new()
        };

        let kind = WolframExportKind::Function(String::new());

        self.consume(&WolframSyntaxKind::RightParen, "Expected ')' ")?;

        Ok(WolframExport { name, kind })
    }

    /// 解析导入
    fn parse_import(&mut self) -> Result<WolframImport, OakError> {
        let module = if self.check(&WolframSyntaxKind::String) {
            self.advance();
            String::new()
        }
        else {
            String::new()
        };
        let name = if self.check(&WolframSyntaxKind::String) {
            self.advance();
            String::new()
        }
        else {
            String::new()
        };

        let kind = WolframImportKind::Function(WolframFunctionType { params: Vec::new(), results: Vec::new() });

        self.consume(&WolframSyntaxKind::RightParen, "Expected ')' ")?;

        Ok(WolframImport { module, name, kind })
    }

    // ======== 通用解析辅助 ========

    fn advance(&mut self) -> &WolframToken {
        let idx = self.position;
        if self.position < self.tokens.len() {
            self.position += 1;
        }
        &self.tokens[idx]
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len() || self.tokens[self.position].kind == WolframSyntaxKind::Eof
    }

    fn check(&self, kind: &WolframSyntaxKind) -> bool {
        !self.is_at_end() && self.tokens[self.position].kind == *kind
    }

    fn match_token(&mut self, kind: &WolframSyntaxKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        }
        else {
            false
        }
    }

    fn consume(&mut self, kind: &WolframSyntaxKind, msg: &str) -> Result<(), OakError> {
        if self.check(kind) {
            self.advance();
            Ok(())
        }
        else {
            Err(self.make_error(msg))
        }
    }

    fn make_error(&self, message: &str) -> OakError {
        let token =
            if self.is_at_end() { &self.tokens[self.tokens.len().saturating_sub(1)] } else { &self.tokens[self.position] };
        let source = SourceLocation {
            line: 1, // TODO: Calculate line from span
            column: token.span.start as u32,
            url: None,
        };
        OakError::syntax_error(message, source)
    }

    /// Get text content of a kind (placeholder implementation)
    fn _get_token_text(&self, _token: &WolframToken) -> String {
        // TODO: Extract text from source using kind.span
        // For now, return empty string as placeholder
        String::new()
    }
}

impl Default for WolframParser {
    fn default() -> Self {
        Self::new()
    }
}
