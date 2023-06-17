use crate::{kind::TclSyntaxKind, language::TclLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, TclLanguage>;

pub struct TclLexer<'config> {
    config: &'config TclLanguage,
}

impl<'config> TclLexer<'config> {
    pub fn new(config: &'config TclLanguage) -> Self {
        Self { config }
    }

    /// 跳过空白字符
    fn skip_whitespace(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();
        let mut found_whitespace = false;

        while let Some(ch) = state.peek() {
            if ch.is_whitespace() && ch != '\n' && ch != '\r' {
                state.advance(ch.len_utf8());
                found_whitespace = true;
            }
            else {
                break;
            }
        }

        if found_whitespace {
            state.add_token(TclSyntaxKind::Whitespace, start_pos, state.get_position());
        }
        found_whitespace
    }

    /// 处理换行符
    fn lex_newline(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch == '\n' {
                state.advance(1);
                state.add_token(TclSyntaxKind::Newline, start_pos, state.get_position());
                return true;
            }
            else if ch == '\r' {
                state.advance(1);
                if let Some('\n') = state.peek() {
                    state.advance(1);
                }
                state.add_token(TclSyntaxKind::Newline, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理行注释
    fn lex_line_comment(&self, state: &mut State, _source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some('#') = state.peek() {
            state.advance(1);

            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(TclSyntaxKind::Comment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理块注释（TCL 没有块注释，返回 false）
    fn lex_block_comment(&self, _state: &mut State, _source: &SourceText) -> bool {
        false
    }

    /// 处理字符串字面量
    fn lex_string_literal(&self, state: &mut State, _source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(quote) = state.peek() {
            if quote == '"' {
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
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(TclSyntaxKind::StringLiteral, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理模板字面量（TCL 中的大括号字符串）
    fn lex_template_literal(&self, state: &mut State, _source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some('{') = state.peek() {
            state.advance(1);
            let mut brace_count = 1;

            while let Some(ch) = state.peek() {
                if ch == '{' {
                    brace_count += 1;
                }
                else if ch == '}' {
                    brace_count -= 1;
                    if brace_count == 0 {
                        state.advance(1);
                        break;
                    }
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(TclSyntaxKind::StringLiteral, start_pos, state.get_position());
            return true;
        }
        false
    }

    /// 处理数字字面量
    fn lex_numeric_literal(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit()
                || (ch == '-' && source.get_char_at(state.get_position() + 1).map_or(false, |c| c.is_ascii_digit()))
            {
                if ch == '-' {
                    state.advance(1);
                }

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else if ch == '.' {
                        state.advance(1);
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_digit() {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                        break;
                    }
                    else {
                        break;
                    }
                }

                state.add_token(TclSyntaxKind::Number, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理标识符或关键字
    fn lex_identifier_or_keyword(&self, state: &mut State, source: &SourceText) -> bool {
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

                // 简单的关键字检查
                let text = source.get_text_in((start_pos..state.get_position()).into()).unwrap();
                let kind = match text {
                    "if" => TclSyntaxKind::If,
                    "else" => TclSyntaxKind::Else,
                    "elseif" => TclSyntaxKind::ElseIf,
                    "for" => TclSyntaxKind::For,
                    "while" => TclSyntaxKind::While,
                    "foreach" => TclSyntaxKind::ForEach,
                    "proc" => TclSyntaxKind::Proc,
                    "return" => TclSyntaxKind::Return,
                    "break" => TclSyntaxKind::Break,
                    "continue" => TclSyntaxKind::Continue,
                    "set" => TclSyntaxKind::Set,
                    "unset" => TclSyntaxKind::Unset,
                    "global" => TclSyntaxKind::Global,
                    "upvar" => TclSyntaxKind::Upvar,
                    "variable" => TclSyntaxKind::Variable,
                    _ => TclSyntaxKind::Identifier,
                };

                state.add_token(kind, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理操作符或标点符号
    fn lex_operator_or_punctuation(&self, state: &mut State, _source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let kind = match ch {
                '(' => TclSyntaxKind::LeftParen,
                ')' => TclSyntaxKind::RightParen,
                '[' => TclSyntaxKind::LeftBracket,
                ']' => TclSyntaxKind::RightBracket,
                '{' => TclSyntaxKind::LeftBrace,
                '}' => TclSyntaxKind::RightBrace,
                ';' => TclSyntaxKind::Semicolon,
                ',' => TclSyntaxKind::Comma,
                '$' => TclSyntaxKind::Dollar,
                '=' => TclSyntaxKind::Equal,
                '+' => TclSyntaxKind::Plus,
                '-' => TclSyntaxKind::Minus,
                '*' => TclSyntaxKind::Star,
                '/' => TclSyntaxKind::Slash,
                '%' => TclSyntaxKind::Percent,
                '<' => TclSyntaxKind::Less,
                '>' => TclSyntaxKind::Greater,
                '!' => TclSyntaxKind::Exclamation,
                '&' => TclSyntaxKind::Ampersand,
                '|' => TclSyntaxKind::Pipe,
                _ => return false,
            };

            state.advance(ch.len_utf8());
            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<TclLanguage> for TclLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<TclSyntaxKind> {
        let mut state = LexerState::new(source);

        while state.not_at_end() {
            // 尝试各种词法规则
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_line_comment(&mut state, source) {
                continue;
            }

            if self.lex_block_comment(&mut state, source) {
                continue;
            }

            if self.lex_string_literal(&mut state, source) {
                continue;
            }

            if self.lex_template_literal(&mut state, source) {
                continue;
            }

            if self.lex_numeric_literal(&mut state, source) {
                continue;
            }

            if self.lex_identifier_or_keyword(&mut state, source) {
                continue;
            }

            if self.lex_operator_or_punctuation(&mut state, source) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(TclSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(TclSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
