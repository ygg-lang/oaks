use crate::{kind::WatSyntaxKind, language::WatLanguage};
use oak_core::{
    IncrementalCache, Lexer, LexerState, OakError,
    lexer::{CommentLine, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<S: Source> = LexerState<S, WatLanguage>;

static WAT_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static WAT_COMMENT: LazyLock<CommentLine> = LazyLock::new(|| CommentLine { line_markers: &[";;"] });
static WAT_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });

#[derive(Clone)]
pub struct WatLexer<'config> {
    config: &'config WatLanguage,
}

impl<'config> Lexer<WatLanguage> for WatLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<WatLanguage>,
    ) -> LexOutput<WatLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> WatLexer<'config> {
    pub fn new(config: &'config WatLanguage) -> Self {
        Self { config }
    }

    fn run<S: Source>(&self, state: &mut State<S>) -> Result<(), OakError> {
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

            state.safe_check(safe_point);
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(WatSyntaxKind::Eof, eof_pos, eof_pos);
        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        match WAT_WHITESPACE.scan(state.rest(), state.get_position(), WatSyntaxKind::Whitespace) {
            Some(token) => {
                state.advance_with(token);
                true
            }
            None => false,
        }
    }

    /// 跳过注释
    fn skip_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        match WAT_COMMENT.scan(state.rest(), state.get_position(), WatSyntaxKind::Comment) {
            Some(token) => {
                state.advance_with(token);
                true
            }
            None => false,
        }
    }

    /// 解析字符串字面量
    fn lex_string_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        match WAT_STRING.scan(state.rest(), start, WatSyntaxKind::StringLiteral) {
            Some(token) => {
                state.advance_with(token);
                true
            }
            None => false,
        }
    }

    /// 解析数字字面量
    fn lex_number_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let ch = match state.current() {
            Some(ch) => ch,
            None => return false,
        };

        if !ch.is_ascii_digit() && ch != '-' && ch != '+' {
            return false;
        }

        let mut pos = 0;
        let text = state.rest();
        let mut chars = text.chars();

        // 处理符号
        if let Some(first_char) = chars.next() {
            if first_char == '-' || first_char == '+' {
                pos += first_char.len_utf8();
            }
            else if first_char.is_ascii_digit() {
                pos += first_char.len_utf8();
            }
            else {
                return false;
            }
        }

        // 处理数字
        for ch in chars {
            if ch.is_ascii_digit() || ch == '.' || ch == 'e' || ch == 'E' || ch == 'x' || ch == 'X' {
                pos += ch.len_utf8();
            }
            else {
                break;
            }
        }

        if pos > 0 {
            state.add_token(WatSyntaxKind::IntegerLiteral, start, start + pos);
            state.advance(pos);
            true
        }
        else {
            false
        }
    }

    /// 解析标识符或关键字
    fn lex_identifier_or_keyword<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let ch = match state.current() {
            Some(ch) => ch,
            None => return false,
        };

        if !ch.is_alphabetic() && ch != '_' && ch != '$' {
            return false;
        }

        let mut pos = 0;
        let text = state.rest();
        for ch in text.chars() {
            if ch.is_alphanumeric() || ch == '_' || ch == '$' || ch == '.' {
                pos += ch.len_utf8();
            }
            else {
                break;
            }
        }

        if pos == 0 {
            return false;
        }

        let word = &text[..pos];
        let kind = match word {
            // 模块结构
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

            // 控制流
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

            // 变量操作
            "local" => WatSyntaxKind::LocalKw,
            "local.get" => WatSyntaxKind::LocalGetKw,
            "local.set" => WatSyntaxKind::LocalSetKw,
            "local.tee" => WatSyntaxKind::LocalTeeKw,
            "global.get" => WatSyntaxKind::GlobalGetKw,
            "global.set" => WatSyntaxKind::GlobalSetKw,

            // 内存操作
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

            // 常量
            "i32.const" => WatSyntaxKind::I32ConstKw,
            "i64.const" => WatSyntaxKind::I64ConstKw,
            "f32.const" => WatSyntaxKind::F32ConstKw,
            "f64.const" => WatSyntaxKind::F64ConstKw,

            // 算术运算
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

            // 比较运算
            "i32.eq" => WatSyntaxKind::I32EqKw,
            "i64.eq" => WatSyntaxKind::I64EqKw,
            "f32.eq" => WatSyntaxKind::F32EqKw,
            "f64.eq" => WatSyntaxKind::F64EqKw,
            "i32.ne" => WatSyntaxKind::I32NeKw,
            "i64.ne" => WatSyntaxKind::I64NeKw,
            "f32.ne" => WatSyntaxKind::F32NeKw,
            "f64.ne" => WatSyntaxKind::F64NeKw,

            // 其他操作
            "drop" => WatSyntaxKind::DropKw,
            "select" => WatSyntaxKind::SelectKw,
            "unreachable" => WatSyntaxKind::UnreachableKw,
            "nop" => WatSyntaxKind::NopKw,

            // 类型关键字
            "i32" => WatSyntaxKind::I32Kw,
            "i64" => WatSyntaxKind::I64Kw,
            "f32" => WatSyntaxKind::F32Kw,
            "f64" => WatSyntaxKind::F64Kw,

            _ => WatSyntaxKind::Identifier,
        };

        state.add_token(kind, start, start + pos);
        state.advance(pos);
        true
    }

    /// 解析标点符号
    fn lex_punctuation<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let ch = match state.current() {
            Some(ch) => ch,
            None => return false,
        };

        let kind = match ch {
            '(' => WatSyntaxKind::LeftParen,
            ')' => WatSyntaxKind::RightParen,
            '=' => WatSyntaxKind::Eq,
            _ => return false,
        };

        state.add_token(kind, start, start + ch.len_utf8());
        state.advance(ch.len_utf8());
        true
    }

    /// 处理无法匹配的文本
    fn lex_text<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.current() {
            state.add_token(WatSyntaxKind::Error, start, start + ch.len_utf8());
            state.advance(ch.len_utf8());
            true
        }
        else {
            false
        }
    }
}
