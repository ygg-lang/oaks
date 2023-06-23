#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::WatLanguage, lexer::token_type::WatTokenType};
use oak_core::{
    Lexer, LexerCache, LexerState, OakError,
    lexer::{CommentConfig, LexOutput, StringConfig, WhitespaceConfig},
    source::{Source, TextEdit},
};
use std::sync::LazyLock;

type State<'a, S> = LexerState<'a, S, WatLanguage>;

static WAT_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static WAT_COMMENT: LazyLock<CommentConfig> = LazyLock::new(|| CommentConfig { line_marker: ";;", block_start: "(;", block_end: ")", nested_blocks: true });
static WAT_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });

#[derive(Clone)]
pub struct WatLexer<'config> {
    _config: &'config WatLanguage,
}

impl<'config> Lexer<WatLanguage> for WatLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, text: &'a S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<WatLanguage>) -> LexOutput<WatLanguage> {
        let mut state = State::new(text);
        let result = self.run(&mut state);
        state.finish_with_cache(result, cache)
    }
}

impl<'config> WatLexer<'config> {
    pub fn new(config: &'config WatLanguage) -> Self {
        Self { _config: config }
    }

    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();
            if self.skip_whitespace(state) {
                continue;
            }

            if self.skip_comment(state) {
                continue;
            }

            if self.lex_string_literal(state) {
                continue;
            }

            if self.lex_number_literal(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_punctuation(state) {
                continue;
            }

            if self.lex_text(state) {
                continue;
            }

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        WAT_WHITESPACE.scan(state, WatTokenType::Whitespace)
    }

    /// 跳过注释
    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        WAT_COMMENT.scan(state, WatTokenType::Comment, WatTokenType::Comment)
    }

    /// 解析字符串字面量
    fn lex_string_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        WAT_STRING.scan(state, WatTokenType::StringLiteral)
    }

    /// 解析数字字面量
    fn lex_number_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() || ch == '-' || ch == '+' {
                state.bump();
                let mut is_float = false;
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() || ch == '_' {
                        state.bump();
                    }
                    else if ch == '.' {
                        is_float = true;
                        state.bump();
                    }
                    else if ch == 'e' || ch == 'E' || ch == 'p' || ch == 'P' || ch == 'x' || ch == 'X' || (ch >= 'a' && ch <= 'f') || (ch >= 'A' && ch <= 'F') {
                        state.bump();
                    }
                    else {
                        break;
                    }
                }
                let kind = if is_float { WatTokenType::FloatLiteral } else { WatTokenType::IntegerLiteral };
                state.add_token(kind, start, state.get_position());
                return true;
            }
        }
        false
    }

    /// 解析标识符或关键字
    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.peek() {
            if ch == '$' || ch.is_ascii_alphabetic() || ch == '_' {
                state.bump();
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '.' || ch == '$' || ch == '-' {
                        state.bump();
                    }
                    else {
                        break;
                    }
                }
                let end = state.get_position();
                let text = state.get_text_in((start..end).into());
                let kind = if text.starts_with('$') {
                    WatTokenType::Identifier
                }
                else {
                    match text.as_ref() {
                        "module" => WatTokenType::ModuleKw,
                        "func" => WatTokenType::FuncKw,
                        "param" => WatTokenType::ParamKw,
                        "result" => WatTokenType::ResultKw,
                        "export" => WatTokenType::ExportKw,
                        "import" => WatTokenType::ImportKw,
                        "table" => WatTokenType::TableKw,
                        "memory" => WatTokenType::MemoryKw,
                        "global" => WatTokenType::GlobalKw,
                        "type" => WatTokenType::TypeKw,
                        "elem" => WatTokenType::ElemKw,
                        "data" => WatTokenType::DataKw,
                        "start" => WatTokenType::StartKw,
                        "block" => WatTokenType::BlockKw,
                        "loop" => WatTokenType::LoopKw,
                        "if" => WatTokenType::IfKw,
                        "then" => WatTokenType::ThenKw,
                        "else" => WatTokenType::ElseKw,
                        "end" => WatTokenType::EndKw,
                        "br" => WatTokenType::BrKw,
                        "br_if" => WatTokenType::BrIfKw,
                        "br_table" => WatTokenType::BrTableKw,
                        "return" => WatTokenType::ReturnKw,
                        "call" => WatTokenType::CallKw,
                        "call_indirect" => WatTokenType::CallIndirectKw,
                        "local" => WatTokenType::LocalKw,
                        "local.get" => WatTokenType::LocalGetKw,
                        "local.set" => WatTokenType::LocalSetKw,
                        "local.tee" => WatTokenType::LocalTeeKw,
                        "global.get" => WatTokenType::GlobalGetKw,
                        "global.set" => WatTokenType::GlobalSetKw,
                        "i32.load" => WatTokenType::I32LoadKw,
                        "i64.load" => WatTokenType::I64LoadKw,
                        "f32.load" => WatTokenType::F32LoadKw,
                        "f64.load" => WatTokenType::F64LoadKw,
                        "i32.store" => WatTokenType::I32StoreKw,
                        "i64.store" => WatTokenType::I64StoreKw,
                        "f32.store" => WatTokenType::F32StoreKw,
                        "f64.store" => WatTokenType::F64StoreKw,
                        "memory.size" => WatTokenType::MemorySizeKw,
                        "memory.grow" => WatTokenType::MemoryGrowKw,
                        "i32.const" => WatTokenType::I32ConstKw,
                        "i64.const" => WatTokenType::I64ConstKw,
                        "f32.const" => WatTokenType::F32ConstKw,
                        "f64.const" => WatTokenType::F64ConstKw,
                        "i32.add" => WatTokenType::I32AddKw,
                        "i64.add" => WatTokenType::I64AddKw,
                        "f32.add" => WatTokenType::F32AddKw,
                        "f64.add" => WatTokenType::F64AddKw,
                        "i32.sub" => WatTokenType::I32SubKw,
                        "i64.sub" => WatTokenType::I64SubKw,
                        "f32.sub" => WatTokenType::F32SubKw,
                        "f64.sub" => WatTokenType::F64SubKw,
                        "i32.mul" => WatTokenType::I32MulKw,
                        "i64.mul" => WatTokenType::I64MulKw,
                        "f32.mul" => WatTokenType::F32MulKw,
                        "f64.mul" => WatTokenType::F64MulKw,
                        "i32.eq" => WatTokenType::I32EqKw,
                        "i64.eq" => WatTokenType::I64EqKw,
                        "f32.eq" => WatTokenType::F32EqKw,
                        "f64.eq" => WatTokenType::F64EqKw,
                        "i32.ne" => WatTokenType::I32NeKw,
                        "i64.ne" => WatTokenType::I64NeKw,
                        "f32.ne" => WatTokenType::F32NeKw,
                        "f64.ne" => WatTokenType::F64NeKw,
                        "drop" => WatTokenType::DropKw,
                        "select" => WatTokenType::SelectKw,
                        "unreachable" => WatTokenType::UnreachableKw,
                        "nop" => WatTokenType::NopKw,
                        "i32" => WatTokenType::I32Kw,
                        "i64" => WatTokenType::I64Kw,
                        "f32" => WatTokenType::F32Kw,
                        "f64" => WatTokenType::F64Kw,
                        _ => WatTokenType::Identifier,
                    }
                };
                state.add_token(kind, start, end);
                return true;
            }
        }
        false
    }

    /// 解析标点符号
    fn lex_punctuation<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.peek() {
            let kind = match ch {
                '(' => Some(WatTokenType::LeftParen),
                ')' => Some(WatTokenType::RightParen),
                '=' => Some(WatTokenType::Eq),
                _ => None,
            };

            if let Some(kind) = kind {
                state.bump();
                state.add_token(kind, start, state.get_position());
                return true;
            }
        }
        false
    }

    /// 解析普通文本
    fn lex_text<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if let Some(_ch) = state.peek() {
            state.bump();
            state.add_token(WatTokenType::Text, start, state.get_position());
            true
        }
        else {
            false
        }
    }
}
