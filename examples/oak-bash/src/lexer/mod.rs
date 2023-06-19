use crate::{kind::BashSyntaxKind, language::BashLanguage};
use oak_core::{IncrementalCache, Lexer, LexerState, OakError, lexer::LexOutput, source::Source};
use std::sync::LazyLock;

type State<S> = LexerState<S, BashLanguage>;

#[derive(Clone)]
pub struct BashLexer<'config> {
    config: &'config BashLanguage,
}

impl<'config> BashLexer<'config> {
    pub fn new(config: &'config BashLanguage) -> Self {
        Self { config }
    }

    fn run<S: Source>(&self, state: &mut State<S>) -> Result<(), OakError> {
        while state.not_at_end() {
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

            // 如果没有匹配任何模式，跳过一个字符
            state.advance(1);
        }
        Ok(())
    }

    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
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
            state.add_token(BashSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn skip_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('#') = state.peek() {
            state.advance(1);
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(BashSyntaxKind::Comment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_newline<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(BashSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(BashSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_string<S: Source>(&self, state: &mut State<S>) -> bool {
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

                state.add_token(BashSyntaxKind::StringLiteral, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_variable<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('$') = state.peek() {
            state.advance(1);

            // 处理特殊变量 $0, $1, $?, $$ 等
            if let Some(ch) = state.peek() {
                if ch.is_ascii_digit() || ch == '?' || ch == '$' || ch == '#' || ch == '@' || ch == '*' {
                    state.advance(1);
                    state.add_token(BashSyntaxKind::Variable, start_pos, state.get_position());
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
                state.add_token(BashSyntaxKind::Variable, start_pos, state.get_position());
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
                    state.add_token(BashSyntaxKind::Variable, start_pos, state.get_position());
                    return true;
                }
            }

            // 如果只有 $ 没有有效变量名，回退
            state.set_position(start_pos);
        }

        false
    }

    fn lex_number<S: Source>(&self, state: &mut State<S>) -> bool {
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
                state.add_token(BashSyntaxKind::NumberLiteral, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_keyword_or_identifier<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

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

                let text = state.get_text_in((start_pos..state.get_position()).into());
                let kind = if BASH_KEYWORDS.contains(&text) { BashSyntaxKind::Keyword } else { BashSyntaxKind::Identifier };

                state.add_token(kind, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_operator_or_delimiter<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let two_char = if let Some(next_ch) = state.peek_next_n(1) { format!("{}{}", ch, next_ch) } else { String::new() };

            // 检查双字符操作符
            if BASH_TWO_CHAR_OPERATORS.contains(&two_char.as_str()) {
                state.advance(2);
                state.add_token(BashSyntaxKind::Operator, start_pos, state.get_position());
                return true;
            }

            // 检查单字符操作符和分隔符
            let ch_str = ch.to_string();
            if BASH_OPERATORS.contains(&ch_str.as_str()) {
                state.advance(1);
                state.add_token(BashSyntaxKind::Operator, start_pos, state.get_position());
                return true;
            }

            if BASH_DELIMITERS.contains(&ch_str.as_str()) {
                state.advance(1);
                state.add_token(BashSyntaxKind::Delimiter, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_heredoc<S: Source>(&self, state: &mut State<S>) -> bool {
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

                state.add_token(BashSyntaxKind::Heredoc, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_glob_pattern<S: Source>(&self, state: &mut State<S>) -> bool {
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

                state.add_token(BashSyntaxKind::GlobPattern, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_special_char<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if BASH_SPECIAL_CHARS.contains(&ch) {
                state.advance(1);
                state.add_token(BashSyntaxKind::SpecialChar, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_text<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if !ch.is_whitespace() && !BASH_SPECIAL_CHARS.contains(&ch) {
                state.advance(ch.len_utf8());
                state.add_token(BashSyntaxKind::Text, start_pos, state.get_position());
                return true;
            }
        }

        false
    }
}

impl<'config> Lexer<BashLanguage> for BashLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        _changed: usize,
        _cache: IncrementalCache<BashLanguage>,
    ) -> LexOutput<BashLanguage> {
        let mut state = LexerState::new_with_cache(source, _changed, _cache);
        let result = self.run(&mut state);
        if result.is_ok() {
            let eof_pos = state.get_position();
            state.add_token(BashSyntaxKind::Eof, eof_pos, eof_pos);
        }
        state.finish(result)
    }
}

static BASH_KEYWORDS: LazyLock<&[&str]> = LazyLock::new(|| {
    &[
        "if", "then", "else", "elif", "fi", "case", "esac", "for", "while", "until", "do", "done", "function", "return",
        "break", "continue", "local", "export", "readonly", "declare", "typeset", "unset", "shift", "exit", "source", ".",
        "eval", "exec", "trap", "wait", "jobs", "bg", "fg", "disown", "suspend", "alias", "unalias", "history", "fc", "let",
        "test", "[", "[[", "]]", "time", "coproc", "select", "in",
    ]
});

static BASH_OPERATORS: LazyLock<&[&str]> = LazyLock::new(|| &["+", "-", "*", "/", "%", "=", "!", "<", ">", "&", "|", "^", "~"]);

static BASH_TWO_CHAR_OPERATORS: LazyLock<&[&str]> = LazyLock::new(|| {
    &[
        "==", "!=", "<=", ">=", "&&", "||", "<<", ">>", "++", "--", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "<<=",
        ">>=", "**",
    ]
});

static BASH_DELIMITERS: LazyLock<&[&str]> = LazyLock::new(|| &["(", ")", "{", "}", "[", "]", ";", ",", ":", "."]);

static BASH_SPECIAL_CHARS: LazyLock<&[char]> = LazyLock::new(|| {
    &[
        '\\', '`', '~', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '+', '=', '{', '}', '[', ']', '|', '\\', ':', ';',
        '"', '\'', '<', '>', ',', '.', '?', '/', '!', '`',
    ]
});
