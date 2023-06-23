#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::PowerShellLanguage, lexer::token_type::PowerShellTokenType};
use oak_core::{
    Lexer, LexerCache, LexerState, OakError,
    lexer::LexOutput,
    source::{Source, TextEdit},
};

type State<'a, S> = LexerState<'a, S, PowerShellLanguage>;

#[derive(Clone)]
pub struct PowerShellLexer<'config> {
    _config: &'config PowerShellLanguage,
}

impl<'config> PowerShellLexer<'config> {
    pub fn new(config: &'config PowerShellLanguage) -> Self {
        Self { _config: config }
    }

    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
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

            if self.lex_number(state) {
                continue;
            }

            if self.lex_variable(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_operators_and_punctuation(state) {
                continue;
            }

            // 如果没有匹配任何规则，跳过当前字符
            if let Some(ch) = state.peek() {
                let start_pos = state.get_position();
                state.advance(ch.len_utf8());
                state.add_token(PowerShellTokenType::Error, start_pos, state.get_position());
            }
            else {
                // 如果已到达文件末尾，退出循环
                break;
            }
        }

        // Add EOF token
        let pos = state.get_position();
        state.add_token(PowerShellTokenType::Eof, pos, pos);

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
            state.add_token(PowerShellTokenType::Whitespace, start_pos, state.get_position());
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
            state.add_token(PowerShellTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(PowerShellTokenType::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('#') = state.peek() {
            state.advance(1);
            // 单行注释
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(PowerShellTokenType::Comment, start_pos, state.get_position());
            true
        }
        else if let Some('<') = state.peek() {
            state.advance(1);
            if let Some('#') = state.peek() {
                state.advance(1);
                // 多行注释 <# ... #>
                let mut depth = 1;
                while let Some(ch) = state.peek() {
                    if depth == 0 {
                        break;
                    }
                    if ch == '<' {
                        state.advance(1);
                        if let Some('#') = state.peek() {
                            state.advance(1);
                            depth += 1;
                        }
                    }
                    else if ch == '#' {
                        state.advance(1);
                        if let Some('>') = state.peek() {
                            state.advance(1);
                            depth -= 1;
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }
                state.add_token(PowerShellTokenType::Comment, start_pos, state.get_position());
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

    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(quote_char) = state.peek() {
            if quote_char == '"' || quote_char == '\'' {
                state.advance(1); // 跳过开始引号

                let mut escaped = false;
                while let Some(ch) = state.peek() {
                    if escaped {
                        escaped = false;
                        state.advance(ch.len_utf8());
                    }
                    else if ch == '`' {
                        // PowerShell 使用反引号作为转义字符
                        escaped = true;
                        state.advance(1);
                    }
                    else if ch == quote_char {
                        state.advance(1); // 跳过结束引号
                        break;
                    }
                    else if ch == '\n' || ch == '\r' {
                        // 字符串可以跨行
                        state.advance(ch.len_utf8());
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(PowerShellTokenType::StringLiteral, start_pos, state.get_position());
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

    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                let start_pos = state.get_position();

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
                    state.advance(1);
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

                // 检查科学记数法
                if let Some(ch) = state.peek() {
                    if ch == 'e' || ch == 'E' {
                        state.advance(1);
                        if let Some(ch) = state.peek() {
                            if ch == '+' || ch == '-' {
                                state.advance(1);
                            }
                        }
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

                state.add_token(PowerShellTokenType::NumberLiteral, start_pos, state.get_position());
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

    fn lex_variable<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('$') = state.peek() {
            state.advance(1);

            // 变量名必须以字母或下划线开头
            if let Some(ch) = state.peek() {
                if ch.is_alphabetic() || ch == '_' {
                    state.advance(ch.len_utf8());

                    // 后续字符可以是字母、数字或下划线
                    while let Some(ch) = state.peek() {
                        if ch.is_alphanumeric() || ch == '_' {
                            state.advance(ch.len_utf8());
                        }
                        else {
                            break;
                        }
                    }

                    state.add_token(PowerShellTokenType::Variable, start_pos, state.get_position());
                    true
                }
                else {
                    // 只有 $ 符号，作为操作符处理
                    state.add_token(PowerShellTokenType::Dollar, start_pos, state.get_position());
                    true
                }
            }
            else {
                state.add_token(PowerShellTokenType::Dollar, start_pos, state.get_position());
                true
            }
        }
        else {
            false
        }
    }

    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' {
                let start_pos = state.get_position();
                let mut text = String::new();

                // 读取标识符
                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' || ch == '-' {
                        text.push(ch);
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 检查是否是关键字
                let kind = match text.as_str() {
                    "begin" => PowerShellTokenType::Begin,
                    "break" => PowerShellTokenType::Break,
                    "catch" => PowerShellTokenType::Catch,
                    "class" => PowerShellTokenType::Class,
                    "continue" => PowerShellTokenType::Continue,
                    "data" => PowerShellTokenType::Data,
                    "define" => PowerShellTokenType::Define,
                    "do" => PowerShellTokenType::Do,
                    "dynamicparam" => PowerShellTokenType::DynamicParam,
                    "else" => PowerShellTokenType::Else,
                    "elseif" => PowerShellTokenType::ElseIf,
                    "end" => PowerShellTokenType::End,
                    "exit" => PowerShellTokenType::Exit,
                    "filter" => PowerShellTokenType::Filter,
                    "finally" => PowerShellTokenType::Finally,
                    "for" => PowerShellTokenType::For,
                    "foreach" => PowerShellTokenType::ForEach,
                    "from" => PowerShellTokenType::From,
                    "function" => PowerShellTokenType::Function,
                    "if" => PowerShellTokenType::If,
                    "in" => PowerShellTokenType::In,
                    "param" => PowerShellTokenType::Param,
                    "process" => PowerShellTokenType::Process,
                    "return" => PowerShellTokenType::Return,
                    "switch" => PowerShellTokenType::Switch,
                    "throw" => PowerShellTokenType::Throw,
                    "trap" => PowerShellTokenType::Trap,
                    "try" => PowerShellTokenType::Try,
                    "until" => PowerShellTokenType::Until,
                    "using" => PowerShellTokenType::Using,
                    "var" => PowerShellTokenType::Var,
                    "while" => PowerShellTokenType::While,
                    "workflow" => PowerShellTokenType::Workflow,
                    "true" => PowerShellTokenType::BooleanLiteral,
                    "false" => PowerShellTokenType::BooleanLiteral,
                    "null" => PowerShellTokenType::NullLiteral,
                    _ => PowerShellTokenType::Identifier,
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

    fn lex_operators_and_punctuation<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if let Some(ch) = state.peek() {
            let start_pos = state.get_position();

            let kind = match ch {
                '+' => {
                    state.advance(1);
                    if let Some('+') = state.peek() {
                        state.advance(1);
                        PowerShellTokenType::Plus
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        PowerShellTokenType::Equal
                    }
                    else {
                        PowerShellTokenType::Plus
                    }
                }
                '-' => {
                    state.advance(1);
                    if let Some('-') = state.peek() {
                        state.advance(1);
                        PowerShellTokenType::Minus
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        PowerShellTokenType::Equal
                    }
                    else {
                        PowerShellTokenType::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PowerShellTokenType::Equal
                    }
                    else {
                        PowerShellTokenType::Multiply
                    }
                }
                '/' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PowerShellTokenType::Equal
                    }
                    else {
                        PowerShellTokenType::Divide
                    }
                }
                '%' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PowerShellTokenType::Equal
                    }
                    else {
                        PowerShellTokenType::Modulo
                    }
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PowerShellTokenType::Equal
                    }
                    else {
                        PowerShellTokenType::Equal
                    }
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PowerShellTokenType::NotEqual
                    }
                    else {
                        PowerShellTokenType::Exclamation
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PowerShellTokenType::LessEqual
                    }
                    else {
                        PowerShellTokenType::LessThan
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PowerShellTokenType::GreaterEqual
                    }
                    else {
                        PowerShellTokenType::GreaterThan
                    }
                }
                '&' => {
                    state.advance(1);
                    if let Some('&') = state.peek() {
                        state.advance(1);
                        PowerShellTokenType::And
                    }
                    else {
                        PowerShellTokenType::Ampersand
                    }
                }
                '|' => {
                    state.advance(1);
                    if let Some('|') = state.peek() {
                        state.advance(1);
                        PowerShellTokenType::Or
                    }
                    else {
                        PowerShellTokenType::Pipe
                    }
                }
                '^' => {
                    state.advance(1);
                    PowerShellTokenType::Xor
                }
                '~' => {
                    state.advance(1);
                    PowerShellTokenType::Not
                }
                '?' => {
                    state.advance(1);
                    PowerShellTokenType::Question
                }
                ':' => {
                    state.advance(1);
                    if let Some(':') = state.peek() {
                        state.advance(1);
                        PowerShellTokenType::DoubleColon
                    }
                    else {
                        PowerShellTokenType::Colon
                    }
                }
                ';' => {
                    state.advance(1);
                    PowerShellTokenType::Semicolon
                }
                ',' => {
                    state.advance(1);
                    PowerShellTokenType::Comma
                }
                '.' => {
                    state.advance(1);
                    if let Some('.') = state.peek() {
                        state.advance(1);
                        PowerShellTokenType::DotDot
                    }
                    else {
                        PowerShellTokenType::Dot
                    }
                }
                '(' => {
                    state.advance(1);
                    PowerShellTokenType::LeftParen
                }
                ')' => {
                    state.advance(1);
                    PowerShellTokenType::RightParen
                }
                '[' => {
                    state.advance(1);
                    PowerShellTokenType::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    PowerShellTokenType::RightBracket
                }
                '{' => {
                    state.advance(1);
                    PowerShellTokenType::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    PowerShellTokenType::RightBrace
                }
                '@' => {
                    state.advance(1);
                    PowerShellTokenType::At
                }
                '`' => {
                    state.advance(1);
                    PowerShellTokenType::Backtick
                }
                _ => return false,
            };

            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<PowerShellLanguage> for PowerShellLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<PowerShellLanguage>) -> LexOutput<PowerShellLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        state.finish_with_cache(result, cache)
    }
}
