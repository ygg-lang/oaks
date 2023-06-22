pub mod token_type;

pub use token_type::BashTokenType;

use crate::language::BashLanguage;
use oak_core::{Lexer, LexerCache, LexerState, OakError, lexer::LexOutput, source::Source};
use std::sync::LazyLock;

type State<'a, S> = LexerState<'a, S, BashLanguage>;

#[derive(Clone)]
pub struct BashLexer<'config> {
    _config: &'config BashLanguage,
}

impl<'config> Lexer<BashLanguage> for BashLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[oak_core::source::TextEdit], mut cache: &'a mut impl LexerCache<BashLanguage>) -> LexOutput<BashLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, &mut cache)
    }
}

impl<'config> BashLexer<'config> {
    pub fn new(config: &'config BashLanguage) -> Self {
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

            if self.lex_newline(state) {
                continue;
            }

            if self.lex_string(state) {
                continue;
            }

            if self.lex_variable(state) {
                continue;
            }

            if self.lex_number(state) {
                continue;
            }

            if self.lex_keyword_or_identifier(state) {
                continue;
            }

            if self.lex_operator_or_delimiter(state) {
                continue;
            }

            if self.lex_heredoc(state) {
                continue;
            }

            if self.lex_glob_pattern(state) {
                continue;
            }

            if self.lex_special_char(state) {
                continue;
            }

            if self.lex_text(state) {
                continue;
            }

            // 如果没有匹配任何模式，跳过一个字符并生成 Error token
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(BashTokenType::Error, start_pos, state.get_position());
            }

            state.advance_if_dead_lock(safe_point);
        }
        Ok(())
    }

    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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
            state.add_token(BashTokenType::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('#') = state.peek() {
            state.advance(1);
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(BashTokenType::Comment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(BashTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(BashTokenType::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(quote) = state.peek() {
            if quote == '"' || quote == '\'' {
                state.advance(1);
                let mut escaped = false;

                while let Some(ch) = state.peek() {
                    if escaped {
                        escaped = false;
                        state.advance(ch.len_utf8());
                        continue;
                    }

                    if ch == '\\' {
                        escaped = true;
                        state.advance(1);
                        continue;
                    }

                    if ch == quote {
                        state.advance(1);
                        break;
                    }

                    state.advance(ch.len_utf8());
                }

                state.add_token(BashTokenType::StringLiteral, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_variable<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('$') = state.peek() {
            state.advance(1);

            // 处理特殊变量 $0, $1, $?, $$ 等
            if let Some(ch) = state.peek() {
                if ch.is_ascii_digit() || ch == '?' || ch == '$' || ch == '#' || ch == '@' || ch == '*' {
                    state.advance(1);
                    state.add_token(BashTokenType::Variable, start_pos, state.get_position());
                    return true;
                }
            }

            // 处理 ${var} 形式
            if let Some('{') = state.peek() {
                state.advance(1);
                while let Some(ch) = state.peek() {
                    if ch == '}' {
                        state.advance(1);
                        break;
                    }
                    state.advance(ch.len_utf8());
                }
                state.add_token(BashTokenType::Variable, start_pos, state.get_position());
                return true;
            }

            // 处理普通变量名
            if let Some(ch) = state.peek() {
                if ch.is_alphabetic() || ch == '_' {
                    state.advance(ch.len_utf8());
                    while let Some(ch) = state.peek() {
                        if ch.is_alphanumeric() || ch == '_' {
                            state.advance(ch.len_utf8());
                        }
                        else {
                            break;
                        }
                    }
                    state.add_token(BashTokenType::Variable, start_pos, state.get_position());
                    return true;
                }
            }

            // 如果只有 $ 没有有效变量名，回退
            state.set_position(start_pos);
        }

        false
    }

    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }
                state.add_token(BashTokenType::NumberLiteral, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_keyword_or_identifier<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = state.get_text_in((start_pos..state.get_position()).into());
                let kind = if BASH_KEYWORDS.contains(&text.as_ref()) { BashTokenType::Keyword } else { BashTokenType::Identifier };

                state.add_token(kind, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_operator_or_delimiter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let two_char = if let Some(next_ch) = state.peek_next_n(1) { format!("{}{}", ch, next_ch) } else { String::new() };

            // 检查双字符操作符
            if BASH_TWO_CHAR_OPERATORS.contains(&two_char.as_str()) {
                state.advance(2);
                state.add_token(BashTokenType::Operator, start_pos, state.get_position());
                return true;
            }

            // 检查单字符操作符和分隔符
            let ch_str = ch.to_string();
            if BASH_OPERATORS.contains(&ch_str.as_str()) {
                state.advance(1);
                state.add_token(BashTokenType::Operator, start_pos, state.get_position());
                return true;
            }

            if BASH_DELIMITERS.contains(&ch_str.as_str()) {
                state.advance(1);
                state.add_token(BashTokenType::Delimiter, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_heredoc<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        // 检查 << 开始的 heredoc
        if let Some('<') = state.peek() {
            if let Some('<') = state.peek_next_n(1) {
                state.advance(2);

                // 跳过可选的 -
                if let Some('-') = state.peek() {
                    state.advance(1);
                }

                // 读取标识符
                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                state.add_token(BashTokenType::Heredoc, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_glob_pattern<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch == '*' || ch == '?' || ch == '[' {
                state.advance(1);

                if ch == '[' {
                    // 处理字符类 [abc] 或 [!abc]
                    if let Some('!') = state.peek() {
                        state.advance(1);
                    }
                    while let Some(ch) = state.peek() {
                        if ch == ']' {
                            state.advance(1);
                            break;
                        }
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(BashTokenType::GlobPattern, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_special_char<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if BASH_SPECIAL_CHARS.contains(&ch) {
                state.advance(1);
                state.add_token(BashTokenType::SpecialChar, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_text<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if !ch.is_whitespace() && !BASH_SPECIAL_CHARS.contains(&ch) {
                state.advance(ch.len_utf8());
                state.add_token(BashTokenType::Text, start_pos, state.get_position());
                return true;
            }
        }

        false
    }
}

static BASH_KEYWORDS: LazyLock<&[&str]> = LazyLock::new(|| {
    &[
        "if", "then", "else", "elif", "fi", "case", "esac", "for", "while", "until", "do", "done", "function", "return", "break", "continue", "local", "export", "readonly", "declare", "typeset", "unset", "shift", "exit", "source", ".", "eval", "exec",
        "trap", "wait", "jobs", "bg", "fg", "disown", "suspend", "alias", "unalias", "history", "fc", "let", "test", "[", "[[", "]]", "time", "coproc", "select", "in",
    ]
});

static BASH_OPERATORS: LazyLock<&[&str]> = LazyLock::new(|| &["+", "-", "*", "/", "%", "=", "!", "<", ">", "&", "|", "^", "~"]);

static BASH_TWO_CHAR_OPERATORS: LazyLock<&[&str]> = LazyLock::new(|| &["==", "!=", "<=", ">=", "&&", "||", "<<", ">>", "++", "--", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "<<=", ">>=", "**"]);

static BASH_DELIMITERS: LazyLock<&[&str]> = LazyLock::new(|| &["(", ")", "{", "}", "[", "]", ";", ",", ":", "."]);

static BASH_SPECIAL_CHARS: LazyLock<&[char]> = LazyLock::new(|| &['\\', '`', '~', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '+', '=', '{', '}', '[', ']', '|', '\\', ':', ';', '"', '\'', '<', '>', ',', '.', '?', '/', '!', '`']);
