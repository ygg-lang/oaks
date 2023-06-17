use crate::{kind::VerilogKind, language::VerilogLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, VerilogLanguage>;

pub struct VerilogLexer<'config> {
    config: &'config VerilogLanguage,
}

impl<'config> VerilogLexer<'config> {
    pub fn new(config: &'config VerilogLanguage) -> Self {
        Self { config }
    }

    /// 跳过空白字符
    fn skip_whitespace(&self, state: &mut State) -> bool {
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
            state.add_token(VerilogKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(VerilogKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(VerilogKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            state.advance(1);
            if let Some('/') = state.peek() {
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch == '\n' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }

                state.add_token(VerilogKind::Comment, start_pos, state.get_position());
                true
            }
            else {
                // 回退，这不是注释
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// 处理标识符和关键字
    fn lex_identifier(&self, state: &mut State, source: &SourceText) -> bool {
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

                // 检查是否是关键字
                let kind = if let Some(text) = source.get_text_in((start_pos..state.get_position()).into()) {
                    match text {
                        "module" => VerilogKind::Module,
                        "endmodule" => VerilogKind::Endmodule,
                        "wire" => VerilogKind::Wire,
                        "reg" => VerilogKind::Reg,
                        "input" => VerilogKind::Input,
                        "output" => VerilogKind::Output,
                        "always" => VerilogKind::Always,
                        "begin" => VerilogKind::Begin,
                        "end" => VerilogKind::End,
                        "if" => VerilogKind::If,
                        "else" => VerilogKind::Else,
                        "assign" => VerilogKind::Assign,
                        _ => VerilogKind::Identifier,
                    }
                } else {
                    VerilogKind::Identifier
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

    /// 处理数字
    fn lex_number(&self, state: &mut State) -> bool {
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

                state.add_token(VerilogKind::Number, start_pos, state.get_position());
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

    /// 处理标点符号
    fn lex_punctuation(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let kind = match ch {
                '(' => VerilogKind::LeftParen,
                ')' => VerilogKind::RightParen,
                '[' => VerilogKind::LeftBracket,
                ']' => VerilogKind::RightBracket,
                '{' => VerilogKind::LeftBrace,
                '}' => VerilogKind::RightBrace,
                '.' => VerilogKind::Dot,
                ';' => VerilogKind::Semicolon,
                ',' => VerilogKind::Comma,
                '+' => VerilogKind::Plus,
                '-' => VerilogKind::Minus,
                '*' => VerilogKind::Star,
                '/' => VerilogKind::Slash,
                '=' => VerilogKind::Equal,
                _ => return false,
            };

            state.advance(1);
            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<VerilogLanguage> for VerilogLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<VerilogKind> {
        let mut state = LexerState::new(source);

        while state.not_at_end() {
            // 尝试各种词法规则
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_comment(&mut state) {
                continue;
            }

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_identifier(&mut state, source) {
                continue;
            }

            if self.lex_punctuation(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(VerilogKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(VerilogKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}