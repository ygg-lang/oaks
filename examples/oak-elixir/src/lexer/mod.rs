use crate::{kind::ElixirSyntaxKind, language::ElixirLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, ElixirLanguage>;

pub struct ElixirLexer<'config> {
    config: &'config ElixirLanguage,
}

impl<'config> ElixirLexer<'config> {
    pub fn new(config: &'config ElixirLanguage) -> Self {
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
            state.add_token(ElixirSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(ElixirSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(ElixirSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('#') = state.peek() {
            state.advance(1);

            // 读取到行

            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(ElixirSyntaxKind::Comment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理标识符、原子或关键字
    fn lex_identifier_atom_or_keyword(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        // 检查是否是原子（以 : 开头）
        let is_atom = if let Some(':') = state.peek() {
            state.advance(1);
            true
        }
        else {
            false
        };

        // 标识符必须以字母或下划线开

        if let Some(ch) = state.peek() {
            if !ch.is_alphabetic() && ch != '_' {
                if is_atom {
                    state.set_position(start_pos); // 回退
                }
                return false;
            }
        }
        else {
            if is_atom {
                state.set_position(start_pos); // 回退
            }
            return false;
        }

        // 读取标识符字

        while let Some(ch) = state.peek() {
            if ch.is_alphanumeric() || ch == '_' || ch == '?' || ch == '!' {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        if state.get_position() > start_pos {
            let text = &source.raw[start_pos..state.get_position()];

            if is_atom {
                state.add_token(ElixirSyntaxKind::Atom, start_pos, state.get_position());
            }
            else {
                // 检查是否是关键

                let token_kind = match text {
                    "after" => ElixirSyntaxKind::After,
                    "and" => ElixirSyntaxKind::And,
                    "case" => ElixirSyntaxKind::Case,
                    "catch" => ElixirSyntaxKind::Catch,
                    "cond" => ElixirSyntaxKind::Cond,
                    "def" => ElixirSyntaxKind::Def,
                    "defp" => ElixirSyntaxKind::Defp,
                    "defmodule" => ElixirSyntaxKind::Defmodule,
                    "defstruct" => ElixirSyntaxKind::Defstruct,
                    "defprotocol" => ElixirSyntaxKind::Defprotocol,
                    "defimpl" => ElixirSyntaxKind::Defimpl,
                    "defmacro" => ElixirSyntaxKind::Defmacro,
                    "defmacrop" => ElixirSyntaxKind::Defmacrop,
                    "do" => ElixirSyntaxKind::Do,
                    "else" => ElixirSyntaxKind::Else,
                    "elsif" => ElixirSyntaxKind::Elsif,
                    "end" => ElixirSyntaxKind::End,
                    "false" => ElixirSyntaxKind::False,
                    "fn" => ElixirSyntaxKind::Fn,
                    "if" => ElixirSyntaxKind::If,
                    "in" => ElixirSyntaxKind::In,
                    "not" => ElixirSyntaxKind::Not,
                    "or" => ElixirSyntaxKind::Or,
                    "receive" => ElixirSyntaxKind::Receive,
                    "rescue" => ElixirSyntaxKind::Rescue,
                    "true" => ElixirSyntaxKind::True,
                    "try" => ElixirSyntaxKind::Try,
                    "unless" => ElixirSyntaxKind::Unless,
                    "when" => ElixirSyntaxKind::When,
                    "with" => ElixirSyntaxKind::With,
                    _ => {
                        // 检查是否是变量（以大写字母开头）
                        if text.chars().next().unwrap().is_uppercase() {
                            ElixirSyntaxKind::Variable
                        }
                        else {
                            ElixirSyntaxKind::Identifier
                        }
                    }
                };

                state.add_token(token_kind, start_pos, state.get_position());
            }
            true
        }
        else {
            false
        }
    }

    /// 处理数字
    fn lex_number(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if !ch.is_ascii_digit() {
                return false;
            }
        }
        else {
            return false;
        }

        // 读取整数部分
        while let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);
            }
            else {
                break;
            }
        }

        // 检查小数点
        if let Some('.') = state.peek() {
            let next_pos = state.get_position() + 1;
            if let Some(next_ch) = state.peek_next_n(next_pos) {
                if next_ch.is_ascii_digit() {
                    state.advance(1); // 跳过小数
                    // 读取小数部分
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }
            }
        }

        // 检查科学计数法
        if let Some(ch) = state.peek() {
            if ch == 'e' || ch == 'E' {
                state.advance(1);

                // 可选的符号
                if let Some(sign) = state.peek() {
                    if sign == '+' || sign == '-' {
                        state.advance(1);
                    }
                }

                // 指数部分
                let exp_start = state.get_position();
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                // 如果没有指数数字，回退
                if state.get_position() == exp_start {
                    state.set_position(start_pos);
                    return false;
                }
            }
        }

        state.add_token(ElixirSyntaxKind::Number, start_pos, state.get_position());
        true
    }

    /// 处理字符

    fn lex_string(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        let quote_char = if let Some('"') = state.peek() {
            '"'
        }
        else if let Some('\'') = state.peek() {
            '\''
        }
        else {
            return false;
        };

        state.advance(1); // 跳过开始引

        while let Some(ch) = state.peek() {
            if ch == quote_char {
                state.advance(1); // 跳过结束引号
                state.add_token(ElixirSyntaxKind::String, start_pos, state.get_position());
                return true;
            }
            else if ch == '\\' {
                state.advance(1); // 跳过转义字符
                if let Some(_) = state.peek() {
                    state.advance(1); // 跳过被转义的字符
                }
            }
            else {
                state.advance(ch.len_utf8());
            }
        }

        // 未闭合的字符

        state.add_token(ElixirSyntaxKind::Error, start_pos, state.get_position());
        true
    }

    /// 处理字符字面

    fn lex_character(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('?') = state.peek() {
            state.advance(1);

            if let Some(ch) = state.peek() {
                if ch == '\\' {
                    state.advance(1); // 跳过反斜

                    if let Some(_) = state.peek() {
                        state.advance(1); // 跳过转义字符
                    }
                }
                else {
                    state.advance(ch.len_utf8());
                }

                state.add_token(ElixirSyntaxKind::Character, start_pos, state.get_position());
                true
            }
            else {
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// 处理 sigil
    fn lex_sigil(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('~') = state.peek() {
            state.advance(1);

            // sigil 标识

            if let Some(ch) = state.peek() {
                if ch.is_alphabetic() {
                    state.advance(1);

                    // 分隔

                    if let Some(delimiter) = state.peek() {
                        let closing_delimiter = match delimiter {
                            '(' => ')',
                            '[' => ']',
                            '{' => '}',
                            '<' => '>',
                            '/' => '/',
                            '|' => '|',
                            '"' => '"',
                            '\'' => '\'',
                            _ => delimiter,
                        };

                        state.advance(1); // 跳过开始分隔符

                        // 读取内容直到结束分隔

                        while let Some(ch) = state.peek() {
                            if ch == closing_delimiter {
                                state.advance(1);
                                break;
                            }
                            state.advance(ch.len_utf8());
                        }

                        // 可选的修饰

                        while let Some(ch) = state.peek() {
                            if ch.is_alphabetic() {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }

                        state.add_token(ElixirSyntaxKind::Sigil, start_pos, state.get_position());
                        return true;
                    }
                }
            }

            state.set_position(start_pos);
        }

        false
    }

    /// 处理操作

    fn lex_operator(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '+' => {
                    state.advance(1);
                    if let Some('+') = state.peek() {
                        state.advance(1);
                        ElixirSyntaxKind::PlusPlus
                    }
                    else {
                        ElixirSyntaxKind::Plus
                    }
                }
                '-' => {
                    state.advance(1);
                    if let Some('-') = state.peek() {
                        state.advance(1);
                        ElixirSyntaxKind::MinusMinus
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        ElixirSyntaxKind::Arrow
                    }
                    else {
                        ElixirSyntaxKind::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    if let Some('*') = state.peek() {
                        state.advance(1);
                        ElixirSyntaxKind::StarStar
                    }
                    else {
                        ElixirSyntaxKind::Star
                    }
                }
                '/' => {
                    state.advance(1);
                    ElixirSyntaxKind::Slash
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            ElixirSyntaxKind::EqualEqualEqual
                        }
                        else {
                            ElixirSyntaxKind::EqualEqual
                        }
                    }
                    else if let Some('~') = state.peek() {
                        state.advance(1);
                        ElixirSyntaxKind::MatchOp
                    }
                    else {
                        ElixirSyntaxKind::Equal
                    }
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            ElixirSyntaxKind::NotEqualEqual
                        }
                        else {
                            ElixirSyntaxKind::NotEqual
                        }
                    }
                    else {
                        ElixirSyntaxKind::Exclamation
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        ElixirSyntaxKind::LessEqual
                    }
                    else if let Some('<') = state.peek() {
                        state.advance(1);
                        ElixirSyntaxKind::LeftShift
                    }
                    else {
                        ElixirSyntaxKind::Less
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        ElixirSyntaxKind::GreaterEqual
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        ElixirSyntaxKind::RightShift
                    }
                    else {
                        ElixirSyntaxKind::Greater
                    }
                }
                '&' => {
                    state.advance(1);
                    ElixirSyntaxKind::Ampersand
                }
                '@' => {
                    state.advance(1);
                    ElixirSyntaxKind::At
                }
                '^' => {
                    state.advance(1);
                    ElixirSyntaxKind::Caret
                }
                '|' => {
                    state.advance(1);
                    if let Some('>') = state.peek() {
                        state.advance(1);
                        ElixirSyntaxKind::PipeRight
                    }
                    else if let Some('|') = state.peek() {
                        state.advance(1);
                        ElixirSyntaxKind::PipePipe
                    }
                    else {
                        ElixirSyntaxKind::Pipe
                    }
                }
                '?' => {
                    state.advance(1);
                    ElixirSyntaxKind::Question
                }
                _ => return false,
            };

            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理分隔

    fn lex_delimiter(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => ElixirSyntaxKind::LeftParen,
                ')' => ElixirSyntaxKind::RightParen,
                '{' => ElixirSyntaxKind::LeftBrace,
                '}' => ElixirSyntaxKind::RightBrace,
                '[' => ElixirSyntaxKind::LeftBracket,
                ']' => ElixirSyntaxKind::RightBracket,
                ',' => ElixirSyntaxKind::Comma,
                ';' => ElixirSyntaxKind::Semicolon,
                '.' => ElixirSyntaxKind::Dot,
                ':' => ElixirSyntaxKind::Colon,
                _ => return false,
            };

            state.advance(ch.len_utf8());
            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<ElixirLanguage> for ElixirLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<ElixirSyntaxKind> {
        let mut state = State::new(source);

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

            if self.lex_string(&mut state) {
                continue;
            }

            if self.lex_character(&mut state) {
                continue;
            }

            if self.lex_sigil(&mut state) {
                continue;
            }

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_identifier_atom_or_keyword(&mut state, source) {
                continue;
            }

            if self.lex_operator(&mut state, source) {
                continue;
            }

            if self.lex_delimiter(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，处理错误字符
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(ElixirSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(ElixirSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
