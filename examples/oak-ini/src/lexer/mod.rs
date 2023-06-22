use crate::{kind::IniSyntaxKind, language::IniLanguage};
use oak_core::{
    Lexer, LexerCache, LexerState, OakError,
    lexer::{CommentConfig, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};

type State<'a, S> = LexerState<'a, S, IniLanguage>;

static _INI_WHITESPACE: WhitespaceConfig = WhitespaceConfig { unicode_whitespace: true };
static _INI_COMMENT: CommentConfig = CommentConfig { line_marker: ";", block_start: "", block_end: "", nested_blocks: false };
static _INI_STRING: StringConfig = StringConfig { quotes: &['"', '\''], escape: Some('\\') };

#[derive(Clone, Debug)]
pub struct IniLexer<'config> {
    _config: &'config IniLanguage,
}

impl<'config> Lexer<IniLanguage> for IniLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[oak_core::TextEdit], cache: &'a mut impl LexerCache<IniLanguage>) -> LexOutput<IniLanguage> {
        let mut state: State<'_, S> = State::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> IniLexer<'config> {
    pub fn new(config: &'config IniLanguage) -> Self {
        Self { _config: config }
    }

    /// 主要的词法分析循环
    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if self.skip_whitespace(state) {
                continue;
            }

            if self.lex_newline(state) {
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

            if self.lex_identifier(state) {
                continue;
            }

            if self.lex_punctuation(state) {
                continue;
            }

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }

    /// 跳过空白字符（不包括换行符）
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' || ch == '\r' {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        if state.get_position() > start {
            state.add_token(IniSyntaxKind::Whitespace, start, state.get_position());
            return true;
        }
        false
    }

    /// 处理换行
    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if state.current() == Some('\n') {
            state.advance(1);
            state.add_token(IniSyntaxKind::Newline, start, state.get_position());
            return true;
        }
        false
    }

    /// 跳过注释
    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.current() {
            if ch == ';' || ch == '#' {
                // 跳过注释字符
                state.advance(1);

                // 读取到行尾
                while let Some(ch) = state.peek() {
                    if ch != '\n' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                state.add_token(IniSyntaxKind::Comment, start, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理字符串字面量
    fn lex_string_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(quote_char) = state.current() {
            if quote_char == '"' || quote_char == '\'' {
                // 跳过开始引号
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch != quote_char {
                        if ch == '\\' {
                            state.advance(1); // 转义字符
                            if let Some(_) = state.peek() {
                                state.advance(1); // 被转义的字符
                            }
                        }
                        else {
                            state.advance(ch.len_utf8());
                        }
                    }
                    else {
                        // 找到结束引号
                        state.advance(1);
                        break;
                    }
                }

                state.add_token(IniSyntaxKind::String, start, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理数字字面量
    fn lex_number_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let first = match state.current() {
            Some(c) => c,
            None => return false,
        };

        // 检查是否以数字或负号开始
        if !first.is_ascii_digit() && first != '-' && first != '+' {
            return false;
        }

        // 如果是符号，检查后面是否跟数字
        if first == '-' || first == '+' {
            if let Some(next) = state.peek_next_n(1) {
                if !next.is_ascii_digit() {
                    return false;
                }
            }
            else {
                return false;
            }
        }

        state.advance(1);
        let mut has_dot = false;
        let mut has_exp = false;

        while let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);
            }
            else if ch == '.' && !has_dot && !has_exp {
                has_dot = true;
                state.advance(1);
            }
            else if (ch == 'e' || ch == 'E') && !has_exp {
                has_exp = true;
                state.advance(1);
                // 处理指数符号
                if let Some(sign) = state.peek() {
                    if sign == '+' || sign == '-' {
                        state.advance(1);
                    }
                }
            }
            else {
                break;
            }
        }

        // 检查是否为有效数字
        let end = state.get_position();
        let text = state.get_text_in((start..end).into());

        // 简单验证：不能只是符号或只是点
        if text.as_ref() == "-" || text.as_ref() == "+" || text.as_ref() == "." {
            // 回退
            state.set_position(start);
            return false;
        }

        // 判断是整数还是浮点数
        let kind = if has_dot || has_exp { IniSyntaxKind::Float } else { IniSyntaxKind::Integer };

        state.add_token(kind, start, state.get_position());
        true
    }

    /// 处理标识符
    fn lex_identifier<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let ch = match state.current() {
            Some(c) => c,
            None => return false,
        };

        // 标识符必须以字母或下划线开始
        if !(ch.is_ascii_alphabetic() || ch == '_') {
            return false;
        }

        state.advance(1);
        while let Some(c) = state.current() {
            if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
                state.advance(1);
            }
            else {
                break;
            }
        }

        let end = state.get_position();
        let text = state.get_text_in((start..end).into());

        // 检查是否为布尔值或日期时间
        let kind = match text.to_lowercase().as_str() {
            "true" | "false" => IniSyntaxKind::Boolean,
            _ => {
                if self.is_datetime_like(text.as_ref()) {
                    IniSyntaxKind::DateTime
                }
                else {
                    IniSyntaxKind::Identifier
                }
            }
        };

        state.add_token(kind, start, state.get_position());
        true
    }

    /// 处理标点符号
    fn lex_punctuation<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        // 优先匹配较长的符号
        if state.starts_with("[[") {
            state.advance(2);
            state.add_token(IniSyntaxKind::DoubleLeftBracket, start, state.get_position());
            return true;
        }

        if state.starts_with("]]") {
            state.advance(2);
            state.add_token(IniSyntaxKind::DoubleRightBracket, start, state.get_position());
            return true;
        }

        if let Some(ch) = state.current() {
            let kind = match ch {
                '{' => IniSyntaxKind::LeftBrace,
                '}' => IniSyntaxKind::RightBrace,
                '[' => IniSyntaxKind::LeftBracket,
                ']' => IniSyntaxKind::RightBracket,
                ',' => IniSyntaxKind::Comma,
                '.' => IniSyntaxKind::Dot,
                '=' => IniSyntaxKind::Equal,
                _ => return false,
            };

            state.advance(ch.len_utf8());
            state.add_token(kind, start, state.get_position());
            return true;
        }

        false
    }

    /// 判断是否类似日期时间格式
    fn is_datetime_like(&self, text: &str) -> bool {
        // 简单的日期时间格式检查
        // 支持 ISO 8601 格式：YYYY-MM-DD, YYYY-MM-DDTHH:MM:SS 等
        if text.len() < 8 {
            return false;
        }

        // 检查是否包含日期分隔符
        if text.contains('-') || text.contains(':') || text.contains('T') {
            // 更详细的检查可以在这里添加
            let chars: Vec<char> = text.chars().collect();
            let mut digit_count = 0;
            let mut separator_count = 0;

            for ch in chars {
                if ch.is_ascii_digit() {
                    digit_count += 1;
                }
                else if ch == '-' || ch == ':' || ch == 'T' || ch == 'Z' || ch == '+' {
                    separator_count += 1;
                }
            }

            // 简单启发式：如果数字多于分隔符，可能是日期时间
            digit_count > separator_count && digit_count >= 6
        }
        else {
            false
        }
    }
}
