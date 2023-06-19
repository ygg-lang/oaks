use crate::{kind::OrgModeSyntaxKind, language::OrgModeLanguage};
use oak_core::{
    IncrementalCache, Lexer, LexerState, OakError,
    lexer::{CommentLine, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<S> = LexerState<S, OrgModeLanguage>;

static ORG_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static ORG_COMMENT: LazyLock<CommentLine> = LazyLock::new(|| CommentLine { line_markers: &["#"] });
static ORG_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });

#[derive(Clone)]
pub struct OrgModeLexer<'config> {
    config: &'config OrgModeLanguage,
}

impl<'config> OrgModeLexer<'config> {
    pub fn new(config: &'config OrgModeLanguage) -> Self {
        Self { config }
    }

    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        match ORG_WHITESPACE.scan(state.rest(), state.get_position(), OrgModeSyntaxKind::Whitespace) {
            Some(token) => {
                state.advance_with(token);
                true
            }
            None => false,
        }
    }

    fn skip_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        match ORG_COMMENT.scan(state.rest(), state.get_position(), OrgModeSyntaxKind::Comment) {
            Some(token) => {
                state.advance_with(token);
                true
            }
            None => false,
        }
    }

    fn lex_string<S: Source>(&self, state: &mut State<S>) -> bool {
        match ORG_STRING.scan(state.rest(), state.get_position(), OrgModeSyntaxKind::Text) {
            Some(token) => {
                state.advance_with(token);
                true
            }
            None => false,
        }
    }

    fn lex_heading<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some('*') = state.peek() {
            let start_pos = state.get_position();
            let mut level = 0;

            // 计算星号数量
            while let Some('*') = state.peek() {
                state.advance(1);
                level += 1;
            }

            // 必须跟空格
            if let Some(' ') = state.peek() {
                state.advance(1);

                // 读取标题内容到行尾
                while let Some(ch) = state.peek() {
                    if ch == '\n' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }

                state.add_token(OrgModeSyntaxKind::Heading, start_pos, state.get_position());
                true
            }
            else {
                // 回退
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    fn lex_text_or_keyword<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' {
                let start_pos = state.get_position();
                let mut text = String::new();

                // 读取字母、数字和下划线
                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        text.push(ch);
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 检查是否是关键字
                let kind = match text.as_str() {
                    "TODO" => OrgModeSyntaxKind::Todo,
                    "DONE" => OrgModeSyntaxKind::Done,
                    "NEXT" => OrgModeSyntaxKind::Next,
                    "WAITING" => OrgModeSyntaxKind::Waiting,
                    "CANCELLED" => OrgModeSyntaxKind::Cancelled,
                    _ => OrgModeSyntaxKind::Text,
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

    fn lex_number_or_date<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                let start_pos = state.get_position();
                let mut has_dash = false;
                let mut has_colon = false;

                // 读取数字、破折号和冒号
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else if ch == '-' && !has_dash {
                        has_dash = true;
                        state.advance(1);
                    }
                    else if ch == ':' && !has_colon {
                        has_colon = true;
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                let kind = if has_dash {
                    OrgModeSyntaxKind::Date
                }
                else if has_colon {
                    OrgModeSyntaxKind::Time
                }
                else {
                    OrgModeSyntaxKind::Number
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

    fn lex_link<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some('[') = state.peek() {
            let start_pos = state.get_position();
            state.advance(1);

            if let Some('[') = state.peek() {
                state.advance(1);

                // 读取链接内容直到 ]]
                let mut bracket_count = 0;
                while let Some(ch) = state.peek() {
                    if ch == ']' {
                        state.advance(1);
                        if let Some(']') = state.peek() {
                            state.advance(1);
                            bracket_count += 1;
                            if bracket_count == 1 {
                                break;
                            }
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(OrgModeSyntaxKind::Link, start_pos, state.get_position());
                true
            }
            else {
                // 回退
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    fn lex_priority<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some('[') = state.peek() {
            let start_pos = state.get_position();
            state.advance(1);

            if let Some('#') = state.peek() {
                state.advance(1);

                if let Some(priority_char) = state.peek() {
                    if priority_char == 'A' || priority_char == 'B' || priority_char == 'C' {
                        state.advance(1);

                        if let Some(']') = state.peek() {
                            state.advance(1);

                            let kind = match priority_char {
                                'A' => OrgModeSyntaxKind::PriorityA,
                                'B' => OrgModeSyntaxKind::PriorityB,
                                'C' => OrgModeSyntaxKind::PriorityC,
                                _ => unreachable!(),
                            };

                            state.add_token(kind, start_pos, state.get_position());
                            return true;
                        }
                    }
                }
            }

            // 回退
            state.set_position(start_pos);
            false
        }
        else {
            false
        }
    }

    fn lex_symbols<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some(ch) = state.peek() {
            let start_pos = state.get_position();
            state.advance(ch.len_utf8());

            let kind = match ch {
                '+' => OrgModeSyntaxKind::Plus,
                '-' => OrgModeSyntaxKind::Minus,
                '*' => OrgModeSyntaxKind::Star,
                '#' => OrgModeSyntaxKind::Hash,
                '|' => OrgModeSyntaxKind::Pipe,
                ':' => OrgModeSyntaxKind::Colon,
                '[' => OrgModeSyntaxKind::LeftBracket,
                ']' => OrgModeSyntaxKind::RightBracket,
                '(' => OrgModeSyntaxKind::LeftParen,
                ')' => OrgModeSyntaxKind::RightParen,
                '{' => OrgModeSyntaxKind::LeftBrace,
                '}' => OrgModeSyntaxKind::RightBrace,
                '<' => OrgModeSyntaxKind::LessThan,
                '>' => OrgModeSyntaxKind::GreaterThan,
                '=' => OrgModeSyntaxKind::Equal,
                '_' => OrgModeSyntaxKind::Underscore,
                '~' => OrgModeSyntaxKind::Tilde,
                '/' => OrgModeSyntaxKind::Slash,
                '\\' => OrgModeSyntaxKind::Backslash,
                '\n' => OrgModeSyntaxKind::Newline,
                _ => {
                    // 未知字符，作为文本处理
                    state.add_token(OrgModeSyntaxKind::Text, start_pos, state.get_position());
                    return true;
                }
            };

            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<OrgModeLanguage> for OrgModeLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<OrgModeLanguage>,
    ) -> LexOutput<OrgModeLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> OrgModeLexer<'config> {
    fn run<S: Source>(&self, state: &mut State<S>) -> Result<(), OakError> {
        while state.not_at_end() {
            // 跳过空白字符
            if self.skip_whitespace(state) {
                continue;
            }

            // 处理注释
            if self.skip_comment(state) {
                continue;
            }

            // 处理字符串
            if self.lex_string(state) {
                continue;
            }

            // 处理标题
            if self.lex_heading(state) {
                continue;
            }

            // 处理链接
            if self.lex_link(state) {
                continue;
            }

            // 处理优先级
            if self.lex_priority(state) {
                continue;
            }

            // 处理数字或日期
            if self.lex_number_or_date(state) {
                continue;
            }

            // 处理文本或关键字
            if self.lex_text_or_keyword(state) {
                continue;
            }

            // 处理符号
            if self.lex_symbols(state) {
                continue;
            }

            // 如果没有匹配任何模式，创建错误 token
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(OrgModeSyntaxKind::Error, start_pos, state.get_position());
            }
            else {
                break;
            }
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(OrgModeSyntaxKind::Eof, eof_pos, eof_pos);
        Ok(())
    }
}
