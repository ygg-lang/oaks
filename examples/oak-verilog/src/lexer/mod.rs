use crate::{VerilogKind, language::VerilogLanguage};
use oak_core::{
    IncrementalCache, Lexer, LexerState, OakError,
    lexer::{CommentLine, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<S: Source> = LexerState<S, VerilogLanguage>;

static VL_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static VL_COMMENT: LazyLock<CommentLine> = LazyLock::new(|| CommentLine { line_markers: &["//"] });
static VL_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });

#[derive(Clone)]
pub struct VerilogLexer<'config> {
    config: &'config VerilogLanguage,
}

impl<'config> Lexer<VerilogLanguage> for VerilogLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<VerilogLanguage>,
    ) -> LexOutput<VerilogLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> VerilogLexer<'config> {
    pub fn new(config: &'config VerilogLanguage) -> Self {
        Self { config }
    }

    /// 主要词法分析循环
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

            if self.lex_operators(state) {
                continue;
            }

            if self.lex_single_char_tokens(state) {
                continue;
            }

            state.safe_check(safe_point);
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(VerilogKind::Eof, eof_pos, eof_pos);
        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        match VL_WHITESPACE.scan(state.rest(), state.get_position(), VerilogKind::Whitespace) {
            Some(token) => {
                state.advance_with(token);
                return true;
            }
            None => {}
        }
        false
    }

    /// 跳过注释
    fn skip_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // 行注释: // ... until newline
        if rest.starts_with("//") {
            state.advance(2);
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(VerilogKind::Comment, start, state.get_position());
            return true;
        }

        // 块注释: /* ... */
        if rest.starts_with("/*") {
            state.advance(2);
            while let Some(ch) = state.peek() {
                if ch == '*' && state.peek_next_n(1) == Some('/') {
                    state.advance(2);
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(VerilogKind::Comment, start, state.get_position());
            return true;
        }

        false
    }

    /// 处理字符串字面量
    fn lex_string_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        if state.current() == Some('"') {
            state.advance(1);
            let mut escaped = false;
            while let Some(ch) = state.peek() {
                if ch == '"' && !escaped {
                    state.advance(1); // consume closing quote
                    break;
                }
                state.advance(ch.len_utf8());
                if escaped {
                    escaped = false;
                    continue;
                }
                if ch == '\\' {
                    escaped = true;
                    continue;
                }
                if ch == '\n' || ch == '\r' {
                    break;
                }
            }
            state.add_token(VerilogKind::String, start, state.get_position());
            return true;
        }
        false
    }

    /// 处理数字字面量
    fn lex_number_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let first = match state.current() {
            Some(c) => c,
            None => return false,
        };

        if !first.is_ascii_digit() {
            return false;
        }

        // 基本数字解析
        state.advance(1);
        while let Some(c) = state.peek() {
            if c.is_ascii_digit() || c == '_' {
                state.advance(1);
            }
            else {
                break;
            }
        }

        // 检查是否有进制前缀 (如 'b, 'h, 'o, 'd)
        if state.peek() == Some('\'') {
            state.advance(1);
            if let Some(base_char) = state.peek() {
                if matches!(base_char, 'b' | 'B' | 'h' | 'H' | 'o' | 'O' | 'd' | 'D') {
                    state.advance(1);
                    while let Some(c) = state.peek() {
                        if c.is_ascii_alphanumeric() || c == '_' {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }
            }
        }

        state.add_token(VerilogKind::Number, start, state.get_position());
        true
    }

    /// 处理标识符和关键字
    fn lex_identifier_or_keyword<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let ch = match state.current() {
            Some(c) => c,
            None => return false,
        };

        if !(ch.is_ascii_alphabetic() || ch == '_') {
            return false;
        }

        state.advance(1);
        while let Some(c) = state.current() {
            if c.is_ascii_alphanumeric() || c == '_' {
                state.advance(1);
            }
            else {
                break;
            }
        }

        let end = state.get_position();
        let text = state.get_text_in((start..end).into());
        let kind = match text {
            "module" => VerilogKind::ModuleKw,
            "endmodule" => VerilogKind::EndmoduleKw,
            "wire" => VerilogKind::WireKw,
            "reg" => VerilogKind::RegKw,
            "input" => VerilogKind::InputKw,
            "output" => VerilogKind::OutputKw,
            "always" => VerilogKind::AlwaysKw,
            "begin" => VerilogKind::BeginKw,
            "end" => VerilogKind::EndKw,
            "if" => VerilogKind::IfKw,
            "else" => VerilogKind::ElseKw,
            "assign" => VerilogKind::AssignKw,
            "posedge" => VerilogKind::PosedgeKw,
            "negedge" => VerilogKind::NegedgeKw,
            "case" => VerilogKind::CaseKw,
            "endcase" => VerilogKind::EndcaseKw,
            "default" => VerilogKind::DefaultKw,
            _ => VerilogKind::Identifier,
        };

        state.add_token(kind, start, state.get_position());
        true
    }

    /// 处理操作符
    fn lex_operators<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // 优先匹配长操作符
        let patterns: &[(&str, VerilogKind)] = &[
            ("==", VerilogKind::EqualEqual),
            ("!=", VerilogKind::NotEqual),
            ("<=", VerilogKind::LessEqual),
            (">=", VerilogKind::GreaterEqual),
            ("<<", VerilogKind::LeftShift),
            (">>", VerilogKind::RightShift),
            ("&&", VerilogKind::AndAnd),
            ("||", VerilogKind::OrOr),
        ];

        for (pat, kind) in patterns {
            if rest.starts_with(pat) {
                state.advance(pat.len());
                state.add_token(*kind, start, state.get_position());
                return true;
            }
        }

        if let Some(ch) = state.current() {
            let kind = match ch {
                '+' => Some(VerilogKind::Plus),
                '-' => Some(VerilogKind::Minus),
                '*' => Some(VerilogKind::Star),
                '/' => Some(VerilogKind::Slash),
                '%' => Some(VerilogKind::Percent),
                '=' => Some(VerilogKind::Equal),
                '!' => Some(VerilogKind::Bang),
                '<' => Some(VerilogKind::Less),
                '>' => Some(VerilogKind::Greater),
                '&' => Some(VerilogKind::Ampersand),
                '|' => Some(VerilogKind::Pipe),
                '^' => Some(VerilogKind::Caret),
                '~' => Some(VerilogKind::Tilde),
                _ => None,
            };

            if let Some(k) = kind {
                state.advance(ch.len_utf8());
                state.add_token(k, start, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理单字符标记
    fn lex_single_char_tokens<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.current() {
            let kind = match ch {
                '(' => VerilogKind::LeftParen,
                ')' => VerilogKind::RightParen,
                '{' => VerilogKind::LeftBrace,
                '}' => VerilogKind::RightBrace,
                '[' => VerilogKind::LeftBracket,
                ']' => VerilogKind::RightBracket,
                ',' => VerilogKind::Comma,
                ';' => VerilogKind::Semicolon,
                '.' => VerilogKind::Dot,
                ':' => VerilogKind::Colon,
                '#' => VerilogKind::Hash,
                '@' => VerilogKind::At,
                _ => return false,
            };

            state.advance(ch.len_utf8());
            state.add_token(kind, start, state.get_position());
            return true;
        }
        false
    }
}
