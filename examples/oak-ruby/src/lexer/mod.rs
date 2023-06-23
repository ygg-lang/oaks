#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::RubyLanguage, lexer::token_type::RubyTokenType};
use oak_core::{LexOutput, Lexer, LexerCache, LexerState, OakError, Source, TextEdit};

type State<'a, S> = LexerState<'a, S, RubyLanguage>;

#[derive(Clone, Debug)]
pub struct RubyLexer<'config> {
    _config: &'config RubyLanguage,
}

impl<'config> Lexer<RubyLanguage> for RubyLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<RubyLanguage>) -> LexOutput<RubyLanguage> {
        let mut state: State<'_, S> = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> RubyLexer<'config> {
    pub fn new(config: &'config RubyLanguage) -> Self {
        Self { _config: config }
    }

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

            if self.lex_symbol(state) {
                continue;
            }

            if self.lex_number_literal(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_operators(state) {
                continue;
            }

            if self.lex_single_char_tokens(state) {
                continue;
            }

            state.advance_if_dead_lock(safe_point)
        }

        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' { state.advance(ch.len_utf8()) } else { break }
        }

        if state.get_position() > start_pos {
            state.add_token(RubyTokenType::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(RubyTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1)
            }
            state.add_token(RubyTokenType::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if let Some('#') = state.peek() {
            let start_pos = state.get_position();
            state.advance(1); // 跳过 '#'

            // 读取到行
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8())
            }

            state.add_token(RubyTokenType::Comment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符串字面量
    fn lex_string_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        // 检查是否是字符串开
        let quote_char = match state.peek() {
            Some('"') => '"',
            Some('\'') => '\'',
            Some('`') => '`',
            _ => return false,
        };

        state.advance(1); // 跳过开始引号
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

            if ch == quote_char {
                state.advance(1); // 跳过结束引号
                break;
            }
            else if ch == '\n' || ch == '\r' {
                // Ruby 字符串可以跨多行
                state.advance(ch.len_utf8())
            }
            else {
                state.advance(ch.len_utf8())
            }
        }

        state.add_token(RubyTokenType::StringLiteral, start_pos, state.get_position());
        true
    }

    /// 处理符号
    fn lex_symbol<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if let Some(':') = state.peek() {
            let start_pos = state.get_position();
            state.advance(1); // 跳过 ':'

            // 检查下一个字符是否是标识符开
            if let Some(ch) = state.peek() {
                if ch.is_ascii_alphabetic() || ch == '_' {
                    // 读取标识
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_alphanumeric() || ch == '_' || ch == '?' || ch == '!' { state.advance(1) } else { break }
                    }
                    state.add_token(RubyTokenType::Symbol, start_pos, state.get_position());
                    return true;
                }
                else if ch == '"' || ch == '\'' {
                    // 引号符号
                    let quote = ch;
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
                        else {
                            state.advance(ch.len_utf8())
                        }
                    }
                    state.add_token(RubyTokenType::Symbol, start_pos, state.get_position());
                    return true;
                }
            }
        }
        false
    }

    /// 处理数字字面
    fn lex_number_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if !state.peek().map_or(false, |c| c.is_ascii_digit()) {
            return false;
        }

        let mut is_float = false;

        // 检查进制前缀
        if state.peek() == Some('0') {
            let next_char = state.peek_next_n(1);
            match next_char {
                Some('b') | Some('B') => {
                    state.advance(2); // 跳过 '0b' '0B'
                    // 读取二进制数
                    while let Some(ch) = state.peek() {
                        if ch == '0' || ch == '1' {
                            state.advance(1);
                        }
                        else if ch == '_' {
                            state.advance(1); // 数字分隔
                        }
                        else {
                            break;
                        }
                    }
                }
                Some('o') | Some('O') => {
                    state.advance(2); // 跳过 '0o' '0O'
                    // 读取八进制数
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() && ch < '8' {
                            state.advance(1);
                        }
                        else if ch == '_' {
                            state.advance(1); // 数字分隔
                        }
                        else {
                            break;
                        }
                    }
                }
                Some('x') | Some('X') => {
                    state.advance(2); // 跳过 '0x' '0X'
                    // 读取十六进制数字
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_hexdigit() {
                            state.advance(1);
                        }
                        else if ch == '_' {
                            state.advance(1); // 数字分隔
                        }
                        else {
                            break;
                        }
                    }
                }
                _ => {
                    // 十进制数
                    self.lex_decimal_number(state, &mut is_float)
                }
            }
        }
        else {
            // 十进制数
            self.lex_decimal_number(state, &mut is_float)
        }

        let kind = if is_float { RubyTokenType::FloatLiteral } else { RubyTokenType::IntegerLiteral };

        state.add_token(kind, start_pos, state.get_position());
        true
    }

    /// 处理十进制数
    fn lex_decimal_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, is_float: &mut bool) {
        // 读取整数部分
        while let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);
            }
            else if ch == '_' {
                state.advance(1); // 数字分隔
            }
            else {
                break;
            }
        }

        // 检查小数点
        if state.peek() == Some('.') && state.peek_next_n(1).map_or(false, |c| c.is_ascii_digit()) {
            *is_float = true;
            state.advance(1); // 跳过小数
            // 读取小数部分
            while let Some(ch) = state.peek() {
                if ch.is_ascii_digit() {
                    state.advance(1);
                }
                else if ch == '_' {
                    state.advance(1); // 数字分隔
                }
                else {
                    break;
                }
            }
        }

        // 检查科学计数法
        if let Some('e') | Some('E') = state.peek() {
            *is_float = true;
            state.advance(1);

            // 可选的符号
            if let Some('+') | Some('-') = state.peek() {
                state.advance(1);
            }

            // 指数部分
            while let Some(ch) = state.peek() {
                if ch.is_ascii_digit() {
                    state.advance(1);
                }
                else if ch == '_' {
                    state.advance(1); // 数字分隔
                }
                else {
                    break;
                }
            }
        }
    }

    /// 处理标识符或关键
    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        // 检查第一个字符
        if !state.peek().map_or(false, |c| c.is_ascii_alphabetic() || c == '_') {
            return false;
        }

        // 构建标识符字符串
        let mut buf = String::new();

        // 读取标识符
        while let Some(ch) = state.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '?' || ch == '!' {
                buf.push(ch);
                state.advance(1);
            }
            else {
                break;
            }
        }

        // 检查是否是关键字
        let kind = match buf.as_str() {
            "if" => RubyTokenType::If,
            "unless" => RubyTokenType::Unless,
            "elsif" => RubyTokenType::Elsif,
            "else" => RubyTokenType::Else,
            "case" => RubyTokenType::Case,
            "when" => RubyTokenType::When,
            "then" => RubyTokenType::Then,
            "for" => RubyTokenType::For,
            "while" => RubyTokenType::While,
            "until" => RubyTokenType::Until,
            "break" => RubyTokenType::Break,
            "next" => RubyTokenType::Next,
            "redo" => RubyTokenType::Redo,
            "retry" => RubyTokenType::Retry,
            "return" => RubyTokenType::Return,
            "yield" => RubyTokenType::Yield,
            "def" => RubyTokenType::Def,
            "class" => RubyTokenType::Class,
            "module" => RubyTokenType::Module,
            "end" => RubyTokenType::End,
            "lambda" => RubyTokenType::Lambda,
            "proc" => RubyTokenType::Proc,
            "begin" => RubyTokenType::Begin,
            "rescue" => RubyTokenType::Rescue,
            "ensure" => RubyTokenType::Ensure,
            "raise" => RubyTokenType::Raise,
            "require" => RubyTokenType::Require,
            "load" => RubyTokenType::Load,
            "include" => RubyTokenType::Include,
            "extend" => RubyTokenType::Extend,
            "prepend" => RubyTokenType::Prepend,
            "and" => RubyTokenType::And,
            "or" => RubyTokenType::Or,
            "not" => RubyTokenType::Not,
            "in" => RubyTokenType::In,
            "true" => RubyTokenType::True,
            "false" => RubyTokenType::False,
            "nil" => RubyTokenType::Nil,
            "super" => RubyTokenType::Super,
            "self" => RubyTokenType::Self_,
            "alias" => RubyTokenType::Alias,
            "undef" => RubyTokenType::Undef,
            "defined?" => RubyTokenType::Defined,
            "do" => RubyTokenType::Do,
            _ => RubyTokenType::Identifier,
        };

        state.add_token(kind, start_pos, state.get_position());
        true
    }

    /// 处理操作
    fn lex_operators<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        // 尝试匹配多字符操作符
        let three_char_ops = ["<=>", "===", "**=", "<<=", ">>=", "||=", "&&=", "..."];
        for op in &three_char_ops {
            if state.peek() == op.chars().nth(0) && state.peek_next_n(1) == op.chars().nth(1) && state.peek_next_n(2) == op.chars().nth(2) {
                state.advance(3);
                let kind = match *op {
                    "<=>" => RubyTokenType::Spaceship,
                    "===" => RubyTokenType::EqualEqualEqual,
                    "**=" => RubyTokenType::PowerAssign,
                    "<<=" => RubyTokenType::LeftShiftAssign,
                    ">>=" => RubyTokenType::RightShiftAssign,
                    "||=" => RubyTokenType::OrOrAssign,
                    "&&=" => RubyTokenType::AndAndAssign,
                    "..." => RubyTokenType::DotDotDot,
                    _ => RubyTokenType::Invalid,
                };
                state.add_token(kind, start_pos, state.get_position());
                return true;
            }
        }

        let two_char_ops = ["**", "<<", ">>", "<=", ">=", "==", "!=", "=~", "!~", "&&", "||", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", ".."];
        for op in &two_char_ops {
            if state.peek() == op.chars().nth(0) && state.peek_next_n(1) == op.chars().nth(1) {
                state.advance(2);
                let kind = match *op {
                    "**" => RubyTokenType::Power,
                    "<<" => RubyTokenType::LeftShift,
                    ">>" => RubyTokenType::RightShift,
                    "<=" => RubyTokenType::LessEqual,
                    ">=" => RubyTokenType::GreaterEqual,
                    "==" => RubyTokenType::EqualEqual,
                    "!=" => RubyTokenType::NotEqual,
                    "=~" => RubyTokenType::Match,
                    "!~" => RubyTokenType::NotMatch,
                    "&&" => RubyTokenType::AndAnd,
                    "||" => RubyTokenType::OrOr,
                    "+=" => RubyTokenType::PlusAssign,
                    "-=" => RubyTokenType::MinusAssign,
                    "*=" => RubyTokenType::MultiplyAssign,
                    "/=" => RubyTokenType::DivideAssign,
                    "%=" => RubyTokenType::ModuloAssign,
                    "&=" => RubyTokenType::AndAssign,
                    "|=" => RubyTokenType::OrAssign,
                    "^=" => RubyTokenType::XorAssign,
                    ".." => RubyTokenType::DotDot,
                    _ => RubyTokenType::Invalid,
                };
                state.add_token(kind, start_pos, state.get_position());
                return true;
            }
        }

        // 尝试匹配单字符操作符
        let single_char_ops = ['+', '-', '*', '/', '%', '=', '<', '>', '&', '|', '^', '!', '~', '?'];

        if let Some(ch) = state.peek() {
            if single_char_ops.contains(&ch) {
                state.advance(1);
                let kind = match ch {
                    '+' => RubyTokenType::Plus,
                    '-' => RubyTokenType::Minus,
                    '*' => RubyTokenType::Multiply,
                    '/' => RubyTokenType::Divide,
                    '%' => RubyTokenType::Modulo,
                    '=' => RubyTokenType::Assign,
                    '<' => RubyTokenType::Less,
                    '>' => RubyTokenType::Greater,
                    '&' => RubyTokenType::BitAnd,
                    '|' => RubyTokenType::BitOr,
                    '^' => RubyTokenType::Xor,
                    '!' => RubyTokenType::LogicalNot,
                    '~' => RubyTokenType::Tilde,
                    '?' => RubyTokenType::Question,
                    _ => RubyTokenType::Invalid,
                };
                state.add_token(kind, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理分隔符
    fn lex_single_char_tokens<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        // 检查双冒号
        if state.peek() == Some(':') && state.peek_next_n(1) == Some(':') {
            state.advance(2);
            state.add_token(RubyTokenType::DoubleColon, start_pos, state.get_position());
            return true;
        }

        // 单字符分隔符
        let delimiters = ['(', ')', '[', ']', '{', '}', ',', ';', '.', ':', '@', '$'];

        if let Some(ch) = state.peek() {
            if delimiters.contains(&ch) {
                state.advance(1);
                let kind = match ch {
                    '(' => RubyTokenType::LeftParen,
                    ')' => RubyTokenType::RightParen,
                    '[' => RubyTokenType::LeftBracket,
                    ']' => RubyTokenType::RightBracket,
                    '{' => RubyTokenType::LeftBrace,
                    '}' => RubyTokenType::RightBrace,
                    ',' => RubyTokenType::Comma,
                    ';' => RubyTokenType::Semicolon,
                    '.' => RubyTokenType::Dot,
                    ':' => RubyTokenType::Colon,
                    '@' => RubyTokenType::At,
                    '$' => RubyTokenType::Dollar,
                    _ => RubyTokenType::Invalid,
                };
                state.add_token(kind, start_pos, state.get_position());
                return true;
            }
        }

        // 如果没有匹配任何已知字符，将其标记为 Invalid 并推进位置
        if let Some(_ch) = state.peek() {
            state.advance(1);
            state.add_token(RubyTokenType::Invalid, start_pos, state.get_position());
            return true;
        }

        false
    }
}
