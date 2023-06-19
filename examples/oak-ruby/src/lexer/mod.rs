use crate::{kind::RubySyntaxKind, language::RubyLanguage};
use oak_core::{
    IncrementalCache, Lexer, LexerState, OakError,
    lexer::{CommentLine, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<S: Source> = LexerState<S, RubyLanguage>;

static RUBY_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static RUBY_COMMENT: LazyLock<CommentLine> = LazyLock::new(|| CommentLine { line_markers: &["#"] });
static RUBY_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"', '\'', '`'], escape: Some('\\') });

#[derive(Clone)]
pub struct RubyLexer<'config> {
    config: &'config RubyLanguage,
}

impl<'config> Lexer<RubyLanguage> for RubyLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<RubyLanguage>,
    ) -> LexOutput<RubyLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> RubyLexer<'config> {
    pub fn new(config: &'config RubyLanguage) -> Self {
        Self { config }
    }

    fn run<S: Source>(&self, state: &mut State<S>) -> Result<(), OakError> {
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

            state.safe_check(safe_point);
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(RubySyntaxKind::Eof, eof_pos, eof_pos);
        Ok(())
    }

    /// 跳过空白字符
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
            state.add_token(RubySyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(RubySyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(RubySyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn skip_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some('#') = state.peek() {
            let start_pos = state.get_position();
            state.advance(1); // 跳过 '#'

            // 读取到行
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(RubySyntaxKind::Comment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符串字面量
    fn lex_string_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        // 检查是否是字符串开
        let quote_char = match state.peek() {
            Some('"') => '"',
            Some('\'') => '\'',
            Some('`') => '`',
            _ => return false,
        };

        state.advance(1); // 跳过开始引
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
                state.advance(ch.len_utf8());
            }
            else {
                state.advance(ch.len_utf8());
            }
        }

        state.add_token(RubySyntaxKind::StringLiteral, start_pos, state.get_position());
        true
    }

    /// 处理符号
    fn lex_symbol<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some(':') = state.peek() {
            let start_pos = state.get_position();
            state.advance(1); // 跳过 ':'

            // 检查下一个字符是否是标识符开
            if let Some(ch) = state.peek() {
                if ch.is_ascii_alphabetic() || ch == '_' {
                    // 读取标识
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_alphanumeric() || ch == '_' || ch == '?' || ch == '!' {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                    state.add_token(RubySyntaxKind::Symbol, start_pos, state.get_position());
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
                            state.advance(ch.len_utf8());
                        }
                    }
                    state.add_token(RubySyntaxKind::Symbol, start_pos, state.get_position());
                    return true;
                }
            }
        }
        false
    }

    /// 处理正则表达
    fn lex_regex<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some('/') = state.peek() {
            let start_pos = state.get_position();
            state.advance(1); // 跳过开始的 '/'

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

                if ch == '/' {
                    state.advance(1); // 跳过结束'/'

                    // 读取修饰
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_alphabetic() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                    break;
                }
                else if ch == '\n' || ch == '\r' {
                    // 正则表达式不能跨
                    break;
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            state.add_token(RubySyntaxKind::RegexLiteral, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理数字字面
    fn lex_number_literal<S: Source>(&self, state: &mut State<S>) -> bool {
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
                    self.lex_decimal_number(state, &mut is_float);
                }
            }
        }
        else {
            // 十进制数
            self.lex_decimal_number(state, &mut is_float);
        }

        let kind = if is_float { RubySyntaxKind::FloatLiteral } else { RubySyntaxKind::IntegerLiteral };

        state.add_token(kind, start_pos, state.get_position());
        true
    }

    /// 处理十进制数
    fn lex_decimal_number<S: Source>(&self, state: &mut State<S>, is_float: &mut bool) {
        // 读取整数部分
        while let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);
            }
            else if ch == '_' {
                state.advance(1); // 数字分隔            } else {
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
                    state.advance(1); // 数字分隔                } else {
                    break;
                }
            }
        }
    }

    /// 处理标识符或关键
    fn lex_identifier_or_keyword<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        // 检查第一个字
        if !state.peek().map_or(false, |c| c.is_ascii_alphabetic() || c == '_') {
            return false;
        }

        // 构建标识符字符串
        let mut buf = String::new();

        // 读取标识
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
            "if" => RubySyntaxKind::If,
            "unless" => RubySyntaxKind::Unless,
            "elsif" => RubySyntaxKind::Elsif,
            "else" => RubySyntaxKind::Else,
            "case" => RubySyntaxKind::Case,
            "when" => RubySyntaxKind::When,
            "then" => RubySyntaxKind::Then,
            "for" => RubySyntaxKind::For,
            "while" => RubySyntaxKind::While,
            "until" => RubySyntaxKind::Until,
            "break" => RubySyntaxKind::Break,
            "next" => RubySyntaxKind::Next,
            "redo" => RubySyntaxKind::Redo,
            "retry" => RubySyntaxKind::Retry,
            "return" => RubySyntaxKind::Return,
            "yield" => RubySyntaxKind::Yield,
            "def" => RubySyntaxKind::Def,
            "class" => RubySyntaxKind::Class,
            "module" => RubySyntaxKind::Module,
            "end" => RubySyntaxKind::End,
            "lambda" => RubySyntaxKind::Lambda,
            "proc" => RubySyntaxKind::Proc,
            "begin" => RubySyntaxKind::Begin,
            "rescue" => RubySyntaxKind::Rescue,
            "ensure" => RubySyntaxKind::Ensure,
            "raise" => RubySyntaxKind::Raise,
            "require" => RubySyntaxKind::Require,
            "load" => RubySyntaxKind::Load,
            "include" => RubySyntaxKind::Include,
            "extend" => RubySyntaxKind::Extend,
            "prepend" => RubySyntaxKind::Prepend,
            "and" => RubySyntaxKind::And,
            "or" => RubySyntaxKind::Or,
            "not" => RubySyntaxKind::Not,
            "in" => RubySyntaxKind::In,
            "true" => RubySyntaxKind::True,
            "false" => RubySyntaxKind::False,
            "nil" => RubySyntaxKind::Nil,
            "super" => RubySyntaxKind::Super,
            "self" => RubySyntaxKind::Self_,
            "alias" => RubySyntaxKind::Alias,
            "undef" => RubySyntaxKind::Undef,
            "defined?" => RubySyntaxKind::Defined,
            "do" => RubySyntaxKind::Do,
            _ => RubySyntaxKind::Identifier,
        };

        state.add_token(kind, start_pos, state.get_position());
        true
    }

    /// 处理操作
    fn lex_operators<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        // 尝试匹配多字符操作符
        let three_char_ops = ["<=>", "===", "**=", "<<=", ">>=", "||=", "&&=", "..."];
        for op in &three_char_ops {
            if state.peek() == op.chars().nth(0)
                && state.peek_next_n(1) == op.chars().nth(1)
                && state.peek_next_n(2) == op.chars().nth(2)
            {
                state.advance(3);
                let kind = match *op {
                    "<=>" => RubySyntaxKind::Spaceship,
                    "===" => RubySyntaxKind::EqualEqualEqual,
                    "**=" => RubySyntaxKind::PowerAssign,
                    "<<=" => RubySyntaxKind::LeftShiftAssign,
                    ">>=" => RubySyntaxKind::RightShiftAssign,
                    "||=" => RubySyntaxKind::OrOrAssign,
                    "&&=" => RubySyntaxKind::AndAndAssign,
                    "..." => RubySyntaxKind::DotDotDot,
                    _ => RubySyntaxKind::Invalid,
                };
                state.add_token(kind, start_pos, state.get_position());
                return true;
            }
        }

        let two_char_ops = [
            "**", "<<", ">>", "<=", ">=", "==", "!=", "=~", "!~", "&&", "||", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=",
            "..",
        ];
        for op in &two_char_ops {
            if state.peek() == op.chars().nth(0) && state.peek_next_n(1) == op.chars().nth(1) {
                state.advance(2);
                let kind = match *op {
                    "**" => RubySyntaxKind::Power,
                    "<<" => RubySyntaxKind::LeftShift,
                    ">>" => RubySyntaxKind::RightShift,
                    "<=" => RubySyntaxKind::LessEqual,
                    ">=" => RubySyntaxKind::GreaterEqual,
                    "==" => RubySyntaxKind::EqualEqual,
                    "!=" => RubySyntaxKind::NotEqual,
                    "=~" => RubySyntaxKind::Match,
                    "!~" => RubySyntaxKind::NotMatch,
                    "&&" => RubySyntaxKind::AndAnd,
                    "||" => RubySyntaxKind::OrOr,
                    "+=" => RubySyntaxKind::PlusAssign,
                    "-=" => RubySyntaxKind::MinusAssign,
                    "*=" => RubySyntaxKind::MultiplyAssign,
                    "/=" => RubySyntaxKind::DivideAssign,
                    "%=" => RubySyntaxKind::ModuloAssign,
                    "&=" => RubySyntaxKind::AndAssign,
                    "|=" => RubySyntaxKind::OrAssign,
                    "^=" => RubySyntaxKind::XorAssign,
                    ".." => RubySyntaxKind::DotDot,
                    _ => RubySyntaxKind::Invalid,
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
                    '+' => RubySyntaxKind::Plus,
                    '-' => RubySyntaxKind::Minus,
                    '*' => RubySyntaxKind::Multiply,
                    '/' => RubySyntaxKind::Divide,
                    '%' => RubySyntaxKind::Modulo,
                    '=' => RubySyntaxKind::Assign,
                    '<' => RubySyntaxKind::Less,
                    '>' => RubySyntaxKind::Greater,
                    '&' => RubySyntaxKind::BitAnd,
                    '|' => RubySyntaxKind::BitOr,
                    '^' => RubySyntaxKind::Xor,
                    '!' => RubySyntaxKind::LogicalNot,
                    '~' => RubySyntaxKind::Tilde,
                    '?' => RubySyntaxKind::Question,
                    _ => RubySyntaxKind::Invalid,
                };
                state.add_token(kind, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理分隔
    fn lex_single_char_tokens<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        // 检查双冒号
        if state.peek() == Some(':') && state.peek_next_n(1) == Some(':') {
            state.advance(2);
            state.add_token(RubySyntaxKind::DoubleColon, start_pos, state.get_position());
            return true;
        }

        // 单字符分隔符
        let delimiters = ['(', ')', '[', ']', '{', '}', ',', ';', '.', ':', '@', '$'];

        if let Some(ch) = state.peek() {
            if delimiters.contains(&ch) {
                state.advance(1);
                let kind = match ch {
                    '(' => RubySyntaxKind::LeftParen,
                    ')' => RubySyntaxKind::RightParen,
                    '[' => RubySyntaxKind::LeftBracket,
                    ']' => RubySyntaxKind::RightBracket,
                    '{' => RubySyntaxKind::LeftBrace,
                    '}' => RubySyntaxKind::RightBrace,
                    ',' => RubySyntaxKind::Comma,
                    ';' => RubySyntaxKind::Semicolon,
                    '.' => RubySyntaxKind::Dot,
                    ':' => RubySyntaxKind::Colon,
                    '@' => RubySyntaxKind::At,
                    '$' => RubySyntaxKind::Dollar,
                    _ => RubySyntaxKind::Invalid,
                };
                state.add_token(kind, start_pos, state.get_position());
                return true;
            }
        }

        // 如果没有匹配任何已知字符，将其标记为 Invalid 并推进位置
        if let Some(_ch) = state.peek() {
            state.advance(1);
            state.add_token(RubySyntaxKind::Invalid, start_pos, state.get_position());
            return true;
        }

        false
    }
}
