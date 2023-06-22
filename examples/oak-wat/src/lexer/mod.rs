use crate::{kind::WatSyntaxKind, language::WatLanguage};
use oak_core::{
    Lexer, LexerCache, LexerState, OakError,
    lexer::{CommentConfig, LexOutput, StringConfig, WhitespaceConfig},
    source::{Source, TextEdit},
};
use std::sync::LazyLock;

type State<'a, S> = LexerState<'a, S, WatLanguage>;

static WAT_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static WAT_COMMENT: LazyLock<CommentConfig> = LazyLock::new(|| CommentConfig { line_marker: ";;", block_start: "(;", block_end: ";)", nested_blocks: true });
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
        WAT_WHITESPACE.scan(state, WatSyntaxKind::Whitespace)
    }

    /// 跳过注释
    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        WAT_COMMENT.scan(state, WatSyntaxKind::Comment, WatSyntaxKind::Comment)
    }

    /// 解析字符串字面量
    fn lex_string_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        WAT_STRING.scan(state, WatSyntaxKind::StringLiteral)
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
                let kind = if is_float { WatSyntaxKind::FloatLiteral } else { WatSyntaxKind::IntegerLiteral };
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
                    WatSyntaxKind::Identifier
                }
                else {
                    match text.as_ref() {
                        "module" => WatSyntaxKind::ModuleKw,
                        "func" => WatSyntaxKind::FuncKw,
                        "param" => WatSyntaxKind::ParamKw,
                        "result" => WatSyntaxKind::ResultKw,
                        "export" => WatSyntaxKind::ExportKw,
                        "import" => WatSyntaxKind::ImportKw,
                        "table" => WatSyntaxKind::TableKw,
                        "memory" => WatSyntaxKind::MemoryKw,
                        "global" => WatSyntaxKind::GlobalKw,
                        "type" => WatSyntaxKind::TypeKw,
                        "elem" => WatSyntaxKind::ElemKw,
                        "data" => WatSyntaxKind::DataKw,
                        "start" => WatSyntaxKind::StartKw,
                        "block" => WatSyntaxKind::BlockKw,
                        "loop" => WatSyntaxKind::LoopKw,
                        "if" => WatSyntaxKind::IfKw,
                        "then" => WatSyntaxKind::ThenKw,
                        "else" => WatSyntaxKind::ElseKw,
                        "end" => WatSyntaxKind::EndKw,
                        "br" => WatSyntaxKind::BrKw,
                        "br_if" => WatSyntaxKind::BrIfKw,
                        "br_table" => WatSyntaxKind::BrTableKw,
                        "return" => WatSyntaxKind::ReturnKw,
                        "call" => WatSyntaxKind::CallKw,
                        "call_indirect" => WatSyntaxKind::CallIndirectKw,
                        "local" => WatSyntaxKind::LocalKw,
                        "local.get" => WatSyntaxKind::LocalGetKw,
                        "local.set" => WatSyntaxKind::LocalSetKw,
                        "local.tee" => WatSyntaxKind::LocalTeeKw,
                        "global.get" => WatSyntaxKind::GlobalGetKw,
                        "global.set" => WatSyntaxKind::GlobalSetKw,
                        "i32.load" => WatSyntaxKind::I32LoadKw,
                        "i64.load" => WatSyntaxKind::I64LoadKw,
                        "f32.load" => WatSyntaxKind::F32LoadKw,
                        "f64.load" => WatSyntaxKind::F64LoadKw,
                        "i32.store" => WatSyntaxKind::I32StoreKw,
                        "i64.store" => WatSyntaxKind::I64StoreKw,
                        "f32.store" => WatSyntaxKind::F32StoreKw,
                        "f64.store" => WatSyntaxKind::F64StoreKw,
                        "memory.size" => WatSyntaxKind::MemorySizeKw,
                        "memory.grow" => WatSyntaxKind::MemoryGrowKw,
                        "i32.const" => WatSyntaxKind::I32ConstKw,
                        "i64.const" => WatSyntaxKind::I64ConstKw,
                        "f32.const" => WatSyntaxKind::F32ConstKw,
                        "f64.const" => WatSyntaxKind::F64ConstKw,
                        "i32.add" => WatSyntaxKind::I32AddKw,
                        "i64.add" => WatSyntaxKind::I64AddKw,
                        "f32.add" => WatSyntaxKind::F32AddKw,
                        "f64.add" => WatSyntaxKind::F64AddKw,
                        "i32.sub" => WatSyntaxKind::I32SubKw,
                        "i64.sub" => WatSyntaxKind::I64SubKw,
                        "f32.sub" => WatSyntaxKind::F32SubKw,
                        "f64.sub" => WatSyntaxKind::F64SubKw,
                        "i32.mul" => WatSyntaxKind::I32MulKw,
                        "i64.mul" => WatSyntaxKind::I64MulKw,
                        "f32.mul" => WatSyntaxKind::F32MulKw,
                        "f64.mul" => WatSyntaxKind::F64MulKw,
                        "i32.eq" => WatSyntaxKind::I32EqKw,
                        "i64.eq" => WatSyntaxKind::I64EqKw,
                        "f32.eq" => WatSyntaxKind::F32EqKw,
                        "f64.eq" => WatSyntaxKind::F64EqKw,
                        "i32.ne" => WatSyntaxKind::I32NeKw,
                        "i64.ne" => WatSyntaxKind::I64NeKw,
                        "f32.ne" => WatSyntaxKind::F32NeKw,
                        "f64.ne" => WatSyntaxKind::F64NeKw,
                        "drop" => WatSyntaxKind::DropKw,
                        "select" => WatSyntaxKind::SelectKw,
                        "unreachable" => WatSyntaxKind::UnreachableKw,
                        "nop" => WatSyntaxKind::NopKw,
                        "i32" => WatSyntaxKind::I32Kw,
                        "i64" => WatSyntaxKind::I64Kw,
                        "f32" => WatSyntaxKind::F32Kw,
                        "f64" => WatSyntaxKind::F64Kw,
                        _ => WatSyntaxKind::Identifier,
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
                '(' => Some(WatSyntaxKind::LeftParen),
                ')' => Some(WatSyntaxKind::RightParen),
                '=' => Some(WatSyntaxKind::Eq),
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
            state.add_token(WatSyntaxKind::Text, start, state.get_position());
            true
        }
        else {
            false
        }
    }
}
