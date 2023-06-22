use crate::{kind::OrgModeSyntaxKind, language::OrgModeLanguage};
use oak_core::{
    TextEdit,
    errors::OakError,
    lexer::{CommentConfig, LexOutput, Lexer, LexerCache, LexerState, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<'a, S> = LexerState<'a, S, OrgModeLanguage>;

static ORG_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static ORG_COMMENT: LazyLock<CommentConfig> = LazyLock::new(|| CommentConfig { line_marker: "#", block_start: "", block_end: "", nested_blocks: false });
static ORG_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });

#[derive(Clone, Debug, Default)]
pub struct OrgModeLexer {}

impl OrgModeLexer {
    pub fn new(_config: &OrgModeLanguage) -> Self {
        Self {}
    }

    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        ORG_WHITESPACE.scan(state, OrgModeSyntaxKind::Whitespace)
    }

    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        ORG_COMMENT.scan(state, OrgModeSyntaxKind::Comment, OrgModeSyntaxKind::Comment)
    }

    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        ORG_STRING.scan(state, OrgModeSyntaxKind::Text)
    }

    fn lex_heading<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if let Some('*') = state.peek() {
            let start_pos = state.get_position();

            // 计算星号数量
            while let Some('*') = state.peek() {
                state.advance(1);
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

    fn lex_text_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' {
                let start_pos = state.get_position();
                // 读取字母、数字和下划线
                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }
                let end_pos = state.get_position();
                let text = state.source().get_text_in((start_pos..end_pos).into());
                let kind = match text.as_ref() {
                    "TODO" => OrgModeSyntaxKind::Todo,
                    "DONE" => OrgModeSyntaxKind::Done,
                    "NEXT" => OrgModeSyntaxKind::Next,
                    "WAITING" => OrgModeSyntaxKind::Waiting,
                    "CANCELLED" => OrgModeSyntaxKind::Cancelled,
                    _ => OrgModeSyntaxKind::Text,
                };
                state.add_token(kind, start_pos, end_pos);
                return true;
            }
        }
        false
    }

    fn lex_link<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if state.starts_with("[[") {
            let start_pos = state.get_position();
            state.advance(2);

            // 读取链接内容直到 ]] 或行尾
            while let Some(ch) = state.peek() {
                if state.starts_with("]]") {
                    state.advance(2);
                    state.add_token(OrgModeSyntaxKind::Link, start_pos, state.get_position());
                    return true;
                }
                if ch == '\n' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(OrgModeSyntaxKind::Text, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_priority<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if state.starts_with("[#") {
            let start_pos = state.get_position();
            state.advance(2);

            if let Some(ch) = state.peek() {
                if ch.is_alphabetic() {
                    state.advance(ch.len_utf8());
                    if let Some(']') = state.peek() {
                        state.advance(1);
                        state.add_token(OrgModeSyntaxKind::Priority, start_pos, state.get_position());
                        return true;
                    }
                }
            }

            state.set_position(start_pos);
            false
        }
        else {
            false
        }
    }

    fn lex_number_or_date<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                let start_pos = state.get_position();
                let mut has_dash = false;

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else if ch == '-' {
                        state.advance(1);
                        has_dash = true;
                    }
                    else {
                        break;
                    }
                }

                let kind = if has_dash { OrgModeSyntaxKind::Date } else { OrgModeSyntaxKind::Number };

                state.add_token(kind, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_symbols<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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

impl Lexer<OrgModeLanguage> for OrgModeLexer {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<OrgModeLanguage>) -> LexOutput<OrgModeLanguage> {
        let mut state = State::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl OrgModeLexer {
    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();
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

            state.advance_if_dead_lock(safe_point);
        }
        Ok(())
    }
}
