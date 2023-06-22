//! Lean 语言词法分析器
//!
//! 提供 Lean 语言的词法分析功能，将源代码文本转换为标记流

use crate::{kind::LeanSyntaxKind, language::LeanLanguage};
use oak_core::{Lexer, LexerCache, LexerState, OakError, TextEdit, lexer::LexOutput, source::Source};

type State<'a, S> = LexerState<'a, S, LeanLanguage>;

/// Lean 词法分析器
#[derive(Debug, Clone)]
pub struct LeanLexer<'config> {
    _config: &'config LeanLanguage,
}

impl<'config> Lexer<LeanLanguage> for LeanLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, text: &'a S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<LeanLanguage>) -> LexOutput<LeanLanguage> {
        let mut state = State::new(text);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> LeanLexer<'config> {
    /// 创建新的 Lean 词法分析器
    pub fn new(config: &'config LeanLanguage) -> Self {
        Self { _config: config }
    }

    /// 跳过空白字符
    fn skip_whitespace<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        if state.get_position() > start_pos {
            state.add_token(LeanSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(LeanSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(LeanSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('-') = state.peek() {
            state.advance(1);
            if let Some('-') = state.peek() {
                // 单行注释
                state.advance(1);
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }
                state.add_token(LeanSyntaxKind::Comment, start_pos, state.get_position());
                true
            }
            else {
                // 回退，这是减号操作符
                state.set_position(start_pos);
                false
            }
        }
        else if let Some('/') = state.peek() {
            state.advance(1);
            if let Some('-') = state.peek() {
                // 块注释开始
                state.advance(1);
                let mut depth = 1;
                while depth > 0 && state.not_at_end() {
                    if let Some('/') = state.peek() {
                        state.advance(1);
                        if let Some('-') = state.peek() {
                            state.advance(1);
                            depth += 1;
                        }
                    }
                    else if let Some('-') = state.peek() {
                        state.advance(1);
                        if let Some('/') = state.peek() {
                            state.advance(1);
                            depth -= 1;
                        }
                    }
                    else if let Some(ch) = state.peek() {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }
                state.add_token(LeanSyntaxKind::Comment, start_pos, state.get_position());
                true
            }
            else {
                // 回退，这是除法操作符
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// 处理字符串字面量
    fn lex_string<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('"') = state.peek() {
            state.advance(1);

            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1);
                    break;
                }
                else if ch == '\\' {
                    state.advance(1);
                    if let Some(escaped) = state.peek() {
                        state.advance(escaped.len_utf8());
                    }
                }
                else if ch == '\n' || ch == '\r' {
                    break; // 字符串不能跨行
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }
            state.add_token(LeanSyntaxKind::StringLiteral, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符字面量
    fn lex_char<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\'') = state.peek() {
            state.advance(1);

            while let Some(ch) = state.peek() {
                if ch == '\'' {
                    state.advance(1);
                    break;
                }
                else if ch == '\\' {
                    state.advance(1);
                    if let Some(escaped) = state.peek() {
                        state.advance(escaped.len_utf8());
                    }
                }
                else if ch == '\n' || ch == '\r' {
                    break;
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }
            state.add_token(LeanSyntaxKind::CharLiteral, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理数字字面量
    fn lex_number<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                // 扫描数字
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 检查小数点
                if let Some('.') = state.peek() {
                    state.advance(1); // 跳过小数点
                    if let Some(next_char) = state.peek() {
                        if next_char.is_ascii_digit() {
                            while let Some(ch) = state.peek() {
                                if ch.is_ascii_digit() {
                                    state.advance(ch.len_utf8());
                                }
                                else {
                                    break;
                                }
                            }
                        }
                    }
                }

                // 检查指数部分
                if let Some(ch) = state.peek() {
                    if ch == 'e' || ch == 'E' {
                        state.advance(1);
                        if let Some(sign) = state.peek() {
                            if sign == '+' || sign == '-' {
                                state.advance(1);
                            }
                        }
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_digit() {
                                state.advance(ch.len_utf8());
                            }
                            else {
                                break;
                            }
                        }
                    }
                }

                state.add_token(LeanSyntaxKind::IntegerLiteral, start_pos, state.get_position());
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    /// 处理标识符和关键字
    fn lex_identifier_or_keyword<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' || ch == '\'' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = state.get_text_in(oak_core::Range { start: start_pos, end: state.get_position() });
                let kind = match text.as_ref() {
                    "axiom" => LeanSyntaxKind::Axiom,
                    "constant" => LeanSyntaxKind::Constant,
                    "def" => LeanSyntaxKind::Def,
                    "example" => LeanSyntaxKind::Example,
                    "inductive" => LeanSyntaxKind::Inductive,
                    "lemma" => LeanSyntaxKind::Lemma,
                    "namespace" => LeanSyntaxKind::Namespace,
                    "open" => LeanSyntaxKind::Open,
                    "private" => LeanSyntaxKind::Private,
                    "protected" => LeanSyntaxKind::Protected,
                    "section" => LeanSyntaxKind::Section,
                    "structure" => LeanSyntaxKind::Structure,
                    "theorem" => LeanSyntaxKind::Theorem,
                    "universe" => LeanSyntaxKind::Universe,
                    "variable" => LeanSyntaxKind::Variable,
                    "variables" => LeanSyntaxKind::Variables,
                    "end" => LeanSyntaxKind::End,
                    "import" => LeanSyntaxKind::Import,
                    "export" => LeanSyntaxKind::Export,
                    "prelude" => LeanSyntaxKind::Prelude,
                    "noncomputable" => LeanSyntaxKind::Noncomputable,
                    "partial" => LeanSyntaxKind::Partial,
                    "unsafe" => LeanSyntaxKind::Unsafe,
                    "mutual" => LeanSyntaxKind::Mutual,
                    "where" => LeanSyntaxKind::Where,
                    "have" => LeanSyntaxKind::Have,
                    "show" => LeanSyntaxKind::Show,
                    "suffices" => LeanSyntaxKind::Suffices,
                    "let" => LeanSyntaxKind::Let,
                    "in" => LeanSyntaxKind::In,
                    "if" => LeanSyntaxKind::If,
                    "then" => LeanSyntaxKind::Then,
                    "else" => LeanSyntaxKind::Else,
                    "match" => LeanSyntaxKind::Match,
                    "with" => LeanSyntaxKind::With,
                    "fun" => LeanSyntaxKind::Fun,
                    "do" => LeanSyntaxKind::Do,
                    "for" => LeanSyntaxKind::For,
                    "while" => LeanSyntaxKind::While,
                    "break" => LeanSyntaxKind::Break,
                    "continue" => LeanSyntaxKind::Continue,
                    "return" => LeanSyntaxKind::Return,
                    "try" => LeanSyntaxKind::Try,
                    "catch" => LeanSyntaxKind::Catch,
                    "finally" => LeanSyntaxKind::Finally,
                    "throw" => LeanSyntaxKind::Throw,
                    _ => LeanSyntaxKind::Identifier,
                };

                state.add_token(kind, start_pos, state.get_position());
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    /// 处理操作符和分隔符
    fn lex_operator_or_delimiter<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let kind = match ch {
                '(' => LeanSyntaxKind::LeftParen,
                ')' => LeanSyntaxKind::RightParen,
                '{' => LeanSyntaxKind::LeftBrace,
                '}' => LeanSyntaxKind::RightBrace,
                '[' => LeanSyntaxKind::LeftBracket,
                ']' => LeanSyntaxKind::RightBracket,
                ',' => LeanSyntaxKind::Comma,
                ';' => LeanSyntaxKind::Semicolon,
                '+' => LeanSyntaxKind::Plus,
                '*' => LeanSyntaxKind::Star,
                '/' => LeanSyntaxKind::Slash,
                '%' => LeanSyntaxKind::Percent,
                '^' => LeanSyntaxKind::Caret,
                '#' => LeanSyntaxKind::Hash,
                '&' => LeanSyntaxKind::Ampersand,
                '|' => LeanSyntaxKind::Pipe,
                '~' => LeanSyntaxKind::Tilde,
                '!' => LeanSyntaxKind::Bang,
                '?' => LeanSyntaxKind::Question,
                '@' => LeanSyntaxKind::At,
                '$' => LeanSyntaxKind::Dollar,
                '<' => LeanSyntaxKind::Lt,
                '>' => LeanSyntaxKind::Gt,
                '=' => LeanSyntaxKind::Eq,
                _ => {
                    // 检查多字符操作符
                    match ch {
                        ':' => {
                            state.advance(1);
                            if let Some('=') = state.peek() {
                                state.advance(1);
                                state.add_token(LeanSyntaxKind::ColonEq, start_pos, state.get_position());
                                return true;
                            }
                            else if let Some(':') = state.peek() {
                                state.advance(1);
                                state.add_token(LeanSyntaxKind::ColonColon, start_pos, state.get_position());
                                return true;
                            }
                            else {
                                state.add_token(LeanSyntaxKind::Colon, start_pos, state.get_position());
                                return true;
                            }
                        }
                        '.' => {
                            state.advance(1);
                            if let Some('.') = state.peek() {
                                state.advance(1);
                                state.add_token(LeanSyntaxKind::DotDot, start_pos, state.get_position());
                                return true;
                            }
                            else {
                                state.add_token(LeanSyntaxKind::Dot, start_pos, state.get_position());
                                return true;
                            }
                        }
                        '-' => {
                            state.advance(1);
                            if let Some('>') = state.peek() {
                                state.advance(1);
                                state.add_token(LeanSyntaxKind::Arrow, start_pos, state.get_position());
                                return true;
                            }
                            else {
                                state.add_token(LeanSyntaxKind::Minus, start_pos, state.get_position());
                                return true;
                            }
                        }
                        _ => return false,
                    }
                }
            };

            state.advance(ch.len_utf8());
            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn run<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();
            if self.skip_whitespace(state) {
                continue;
            }

            if self.lex_newline(state) {
                continue;
            }

            if self.lex_comment(state) {
                continue;
            }

            if self.lex_string(state) {
                continue;
            }

            if self.lex_char(state) {
                continue;
            }

            if self.lex_number(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_operator_or_delimiter(state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(LeanSyntaxKind::Error, start_pos, state.get_position());
            }

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }
}
