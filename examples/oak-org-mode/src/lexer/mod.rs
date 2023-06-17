use crate::{kind::OrgModeSyntaxKind, language::OrgModeLanguage};
use alloc::string::ToString;
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, OrgModeLanguage>;

pub struct OrgModeLexer<'config> {
    config: &'config OrgModeLanguage,
}

impl<'config> OrgModeLexer<'config> {
    pub fn new(config: &'config OrgModeLanguage) -> Self {
        Self { config }
    }

    /// 跳过空白字符（除了换行符
    fn skip_whitespace(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' || ch == '\r' {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        if state.get_position() > start_pos {
            state.add_token(OrgModeSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline(&self, state: &mut State) -> bool {
        if let Some('\n') = state.peek() {
            let start_pos = state.get_position();
            state.advance(1);

            // 检查是否是多个连续换行
            while let Some('\n') = state.peek() {
                state.advance(1);
            }

            state.add_token(OrgModeSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut State, source: &SourceText) -> bool {
        if let Some('#') = state.peek() {
            let start_pos = state.get_position();

            // 检查是否是注释行（# 开头）
            let is_line_start = start_pos == 0 || (start_pos > 0 && source.get_char_at(start_pos - 1) == Some('\n'));

            if is_line_start {
                state.advance(1);

                // 读取到行
                while let Some(ch) = state.peek() {
                    if ch == '\n' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }

                state.add_token(OrgModeSyntaxKind::Comment, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理标题
    fn lex_heading(&self, state: &mut State) -> bool {
        if let Some('*') = state.peek() {
            let start_pos = state.get_position();
            let mut star_count = 0;

            // 计算星号数量
            while let Some('*') = state.peek() {
                state.advance(1);
                star_count += 1;
                if star_count > 6 {
                    break;
                }
            }

            // 检查星号后是否有空
            if let Some(' ') = state.peek() {
                let token_kind = match star_count {
                    1 => OrgModeSyntaxKind::HeadingLevel1,
                    2 => OrgModeSyntaxKind::HeadingLevel2,
                    3 => OrgModeSyntaxKind::HeadingLevel3,
                    4 => OrgModeSyntaxKind::HeadingLevel4,
                    5 => OrgModeSyntaxKind::HeadingLevel5,
                    6 => OrgModeSyntaxKind::HeadingLevel6,
                    _ => OrgModeSyntaxKind::Star,
                };

                state.add_token(token_kind, start_pos, state.get_position());
                return true;
            }
            else {
                // 回退，这不是标题
                state.set_position(start_pos);
            }
        }
        false
    }

    /// 处理文本和关键字
    fn lex_text_or_keyword(&self, state: &mut State, source: &SourceText) -> bool {
        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' {
                let start_pos = state.get_position();

                // 读取标识
                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' || ch == '-' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }
                let range = start_pos..state.get_position();
                let text = source.get_text_in(range.into()).unwrap_or("");

                let token_kind = match text {
                    "TODO" => OrgModeSyntaxKind::Todo,
                    "DONE" => OrgModeSyntaxKind::Done,
                    "NEXT" => OrgModeSyntaxKind::Next,
                    "WAITING" => OrgModeSyntaxKind::Waiting,
                    "CANCELLED" => OrgModeSyntaxKind::Cancelled,
                    _ => {
                        // 检查自定义关键
                        if self.config.todo_keywords.contains(&text.to_string()) {
                            OrgModeSyntaxKind::TodoKeyword
                        }
                        else if self.config.done_keywords.contains(&text.to_string()) {
                            OrgModeSyntaxKind::DoneKeyword
                        }
                        else {
                            OrgModeSyntaxKind::Text
                        }
                    }
                };

                state.add_token(token_kind, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理数字和日
    fn lex_number_or_date(&self, state: &mut State, source: &SourceText) -> bool {
        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                let start_pos = state.get_position();

                // 读取数字
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() || ch == '.' || ch == '-' || ch == ':' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }
                let range = start_pos..state.get_position();
                let text = source.get_text_in(range.into()).unwrap_or("");

                // 简单判断是否是日期/时间格式
                let token_kind = if text.contains('-') && text.len() >= 8 {
                    OrgModeSyntaxKind::Date
                }
                else if text.contains(':') {
                    OrgModeSyntaxKind::Time
                }
                else {
                    OrgModeSyntaxKind::Number
                };

                state.add_token(token_kind, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理链接
    fn lex_link(&self, state: &mut State) -> bool {
        if let Some('[') = state.peek() {
            let start_pos = state.get_position();
            state.advance(1);

            if let Some('[') = state.peek() {
                state.advance(1);

                // 读取]]
                let mut bracket_count = 0;
                while let Some(ch) = state.peek() {
                    state.advance(ch.len_utf8());
                    if ch == ']' {
                        bracket_count += 1;
                        if bracket_count == 2 {
                            break;
                        }
                    }
                    else {
                        bracket_count = 0;
                    }
                }

                state.add_token(OrgModeSyntaxKind::Link, start_pos, state.get_position());
                return true;
            }
            else {
                // 回退
                state.set_position(start_pos);
            }
        }
        false
    }

    /// 处理优先
    fn lex_priority(&self, state: &mut State) -> bool {
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

                            let token_kind = match priority_char {
                                'A' => OrgModeSyntaxKind::PriorityA,
                                'B' => OrgModeSyntaxKind::PriorityB,
                                'C' => OrgModeSyntaxKind::PriorityC,
                                _ => unreachable!(),
                            };

                            state.add_token(token_kind, start_pos, state.get_position());
                            return true;
                        }
                    }
                }
            }

            // 回退
            state.set_position(start_pos);
        }
        false
    }

    /// 处理符号
    fn lex_symbols(&self, state: &mut State) -> bool {
        if let Some(ch) = state.peek() {
            let start_pos = state.get_position();
            state.advance(ch.len_utf8());

            let token_kind = match ch {
                '*' => OrgModeSyntaxKind::Star,
                '+' => OrgModeSyntaxKind::Plus,
                '-' => OrgModeSyntaxKind::Minus,
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
                _ => return false,
            };

            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<OrgModeLanguage> for OrgModeLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<OrgModeSyntaxKind> {
        let mut state = LexerState::new(source);

        while state.not_at_end() {
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_comment(&mut state, source) {
                continue;
            }

            if self.lex_heading(&mut state) {
                continue;
            }

            if self.lex_link(&mut state) {
                continue;
            }

            if self.lex_priority(&mut state) {
                continue;
            }

            if self.lex_number_or_date(&mut state, source) {
                continue;
            }

            if self.lex_text_or_keyword(&mut state, source) {
                continue;
            }

            if self.lex_symbols(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(OrgModeSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(OrgModeSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
