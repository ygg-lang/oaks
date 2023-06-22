use crate::{kind::SwiftSyntaxKind, language::SwiftLanguage};
use oak_core::{Lexer, LexerState, OakError, lexer::LexOutput, source::Source};

type State<'a, S> = LexerState<'a, S, SwiftLanguage>;

pub struct SwiftLexer<'config> {
    _config: &'config SwiftLanguage,
}

impl<'config> SwiftLexer<'config> {
    pub fn new(config: &'config SwiftLanguage) -> Self {
        Self { _config: config }
    }

    /// 跳过空白字符
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
            state.add_token(SwiftSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(SwiftSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(SwiftSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            if let Some('/') = state.peek_next_n(1) {
                // 单行注释
                state.advance(2);
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }
                state.add_token(SwiftSyntaxKind::Comment, start_pos, state.get_position());
                true
            }
            else if let Some('*') = state.peek_next_n(1) {
                // 多行注释
                state.advance(2);
                let mut depth = 1;
                while let Some(ch) = state.peek() {
                    if ch == '/'
                        && let Some('*') = state.peek_next_n(1)
                    {
                        state.advance(2);
                        depth += 1;
                    }
                    else if ch == '*'
                        && let Some('/') = state.peek_next_n(1)
                    {
                        state.advance(2);
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }
                state.add_token(SwiftSyntaxKind::Comment, start_pos, state.get_position());
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

    /// 处理标识符或关键
    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        // 处理反引号标识符 `identifier`
        let is_escaped = if let Some('`') = state.peek() {
            state.advance(1);
            true
        }
        else {
            false
        };

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 如果是转义标识符，需要匹配结束的反引
                if is_escaped {
                    if let Some('`') = state.peek() {
                        state.advance(1);
                    }
                    state.add_token(SwiftSyntaxKind::Identifier, start_pos, state.get_position());
                    return true;
                }

                // 检查是否为关键
                let text = state.get_text_in(core::range::Range { start: start_pos, end: state.get_position() });

                let token_kind = match text.as_ref() {
                    "class" => SwiftSyntaxKind::Class,
                    "struct" => SwiftSyntaxKind::Struct,
                    "enum" => SwiftSyntaxKind::Enum,
                    "protocol" => SwiftSyntaxKind::Protocol,
                    "extension" => SwiftSyntaxKind::Extension,
                    "func" => SwiftSyntaxKind::Func,
                    "var" => SwiftSyntaxKind::Var,
                    "let" => SwiftSyntaxKind::Let,
                    "init" => SwiftSyntaxKind::Init,
                    "deinit" => SwiftSyntaxKind::Deinit,
                    "subscript" => SwiftSyntaxKind::Subscript,
                    "typealias" => SwiftSyntaxKind::Typealias,
                    "import" => SwiftSyntaxKind::Import,
                    "if" => SwiftSyntaxKind::If,
                    "else" => SwiftSyntaxKind::Else,
                    "switch" => SwiftSyntaxKind::Switch,
                    "case" => SwiftSyntaxKind::Case,
                    "default" => SwiftSyntaxKind::Default,
                    "for" => SwiftSyntaxKind::For,
                    "while" => SwiftSyntaxKind::While,
                    "repeat" => SwiftSyntaxKind::Repeat,
                    "do" => SwiftSyntaxKind::Do,
                    "break" => SwiftSyntaxKind::Break,
                    "continue" => SwiftSyntaxKind::Continue,
                    "fallthrough" => SwiftSyntaxKind::Fallthrough,
                    "return" => SwiftSyntaxKind::Return,
                    "throw" => SwiftSyntaxKind::Throw,
                    "try" => SwiftSyntaxKind::Try,
                    "catch" => SwiftSyntaxKind::Catch,
                    "finally" => SwiftSyntaxKind::Finally,
                    "guard" => SwiftSyntaxKind::Guard,
                    "defer" => SwiftSyntaxKind::Defer,
                    "public" => SwiftSyntaxKind::Public,
                    "private" => SwiftSyntaxKind::Private,
                    "internal" => SwiftSyntaxKind::Internal,
                    "fileprivate" => SwiftSyntaxKind::Fileprivate,
                    "open" => SwiftSyntaxKind::Open,
                    "static" => SwiftSyntaxKind::Static,
                    "final" => SwiftSyntaxKind::Final,
                    "override" => SwiftSyntaxKind::Override,
                    "mutating" => SwiftSyntaxKind::Mutating,
                    "nonmutating" => SwiftSyntaxKind::Nonmutating,
                    "lazy" => SwiftSyntaxKind::Lazy,
                    "weak" => SwiftSyntaxKind::Weak,
                    "unowned" => SwiftSyntaxKind::Unowned,
                    "optional" => SwiftSyntaxKind::Optional,
                    "required" => SwiftSyntaxKind::Required,
                    "convenience" => SwiftSyntaxKind::Convenience,
                    "dynamic" => SwiftSyntaxKind::Dynamic,
                    "infix" => SwiftSyntaxKind::Infix,
                    "prefix" => SwiftSyntaxKind::Prefix,
                    "postfix" => SwiftSyntaxKind::Postfix,
                    "Any" => SwiftSyntaxKind::Any,
                    "AnyObject" => SwiftSyntaxKind::AnyObject,
                    "Self" => SwiftSyntaxKind::Self_,
                    "Type" => SwiftSyntaxKind::Type,
                    "Protocol" => SwiftSyntaxKind::Protocol_,
                    "true" => SwiftSyntaxKind::True,
                    "false" => SwiftSyntaxKind::False,
                    "nil" => SwiftSyntaxKind::Nil,
                    "as" => SwiftSyntaxKind::As,
                    "is" => SwiftSyntaxKind::Is,
                    "in" => SwiftSyntaxKind::In,
                    "where" => SwiftSyntaxKind::Where,
                    "associatedtype" => SwiftSyntaxKind::Associatedtype,
                    "operator" => SwiftSyntaxKind::Operator,
                    "precedencegroup" => SwiftSyntaxKind::Precedencegroup,
                    "indirect" => SwiftSyntaxKind::Indirect,
                    "rethrows" => SwiftSyntaxKind::Rethrows,
                    "throws" => SwiftSyntaxKind::Throws,
                    "inout" => SwiftSyntaxKind::Inout,
                    _ => SwiftSyntaxKind::Identifier,
                };
                state.add_token(token_kind, start_pos, state.get_position());
                true
            }
            else {
                if is_escaped {
                    // 回退反引
                    state.set_position(start_pos);
                }
                false
            }
        }
        else {
            if is_escaped {
                // 回退反引
                state.set_position(start_pos);
            }
            false
        }
    }

    /// 处理数字字面
    fn lex_number_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);

                // 处理二进制、八进制、十六进制
                if ch == '0' {
                    if let Some('b') | Some('B') = state.peek() {
                        state.advance(1);
                        while let Some(ch) = state.peek() {
                            if ch == '0' || ch == '1' || ch == '_' {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                    }
                    else if let Some('o') | Some('O') = state.peek() {
                        state.advance(1);
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_digit() && ch < '8' || ch == '_' {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                    }
                    else if let Some('x') | Some('X') = state.peek() {
                        state.advance(1);
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_hexdigit() || ch == '_' {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                    }
                    else {
                        // 普通十进制数字
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_digit() || ch == '_' {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                    }
                }
                else {
                    // 十进制数
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() || ch == '_' {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }

                // 处理小数
                if let Some('.') = state.peek() {
                    state.advance(1);
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() || ch == '_' {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }

                // 处理指数
                if let Some('e') | Some('E') = state.peek() {
                    state.advance(1);
                    if let Some('+') | Some('-') = state.peek() {
                        state.advance(1);
                    }
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() || ch == '_' {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }

                state.add_token(SwiftSyntaxKind::NumberLiteral, start_pos, state.get_position());
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

    /// 处理字符串字面量
    fn lex_string_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        // 处理多行字符"""..."""
        if let Some('"') = state.peek() {
            if let Some('"') = state.peek_next_n(1) {
                if let Some('"') = state.peek_next_n(2) {
                    // 多行字符
                    state.advance(3);
                    while let Some(ch) = state.peek() {
                        if ch == '"' {
                            if let Some('"') = state.peek_next_n(1) {
                                if let Some('"') = state.peek_next_n(2) {
                                    state.advance(3);
                                    break;
                                }
                            }
                        }
                        state.advance(ch.len_utf8());
                    }
                    state.add_token(SwiftSyntaxKind::StringLiteral, start_pos, state.get_position());
                    return true;
                }
            }

            // 普通字符串
            state.advance(1);
            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1);
                    break;
                }
                else if ch == '\\' {
                    state.advance(1);
                    if let Some(_) = state.peek() {
                        state.advance(1);
                    }
                }
                else if ch == '\n' || ch == '\r' {
                    break; // 普通字符串不能跨行
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }
            state.add_token(SwiftSyntaxKind::StringLiteral, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理操作
    fn lex_operator<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '+' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        SwiftSyntaxKind::PlusAssign
                    }
                    else {
                        SwiftSyntaxKind::Plus
                    }
                }
                '-' => {
                    state.advance(1);
                    match state.peek() {
                        Some('=') => {
                            state.advance(1);
                            SwiftSyntaxKind::MinusAssign
                        }
                        Some('>') => {
                            state.advance(1);
                            SwiftSyntaxKind::Arrow
                        }
                        _ => SwiftSyntaxKind::Minus,
                    }
                }
                '*' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        SwiftSyntaxKind::StarAssign
                    }
                    else {
                        SwiftSyntaxKind::Star
                    }
                }
                '/' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        SwiftSyntaxKind::SlashAssign
                    }
                    else {
                        SwiftSyntaxKind::Slash
                    }
                }
                '%' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        SwiftSyntaxKind::PercentAssign
                    }
                    else {
                        SwiftSyntaxKind::Percent
                    }
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        SwiftSyntaxKind::Equal
                    }
                    else {
                        SwiftSyntaxKind::Assign
                    }
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        SwiftSyntaxKind::NotEqual
                    }
                    else {
                        SwiftSyntaxKind::LogicalNot
                    }
                }
                '<' => {
                    state.advance(1);
                    match state.peek() {
                        Some('=') => {
                            state.advance(1);
                            SwiftSyntaxKind::LessEqual
                        }
                        Some('<') => {
                            state.advance(1);
                            if let Some('=') = state.peek() {
                                state.advance(1);
                                SwiftSyntaxKind::LeftShiftAssign
                            }
                            else {
                                SwiftSyntaxKind::LeftShift
                            }
                        }
                        _ => SwiftSyntaxKind::Less,
                    }
                }
                '>' => {
                    state.advance(1);
                    match state.peek() {
                        Some('=') => {
                            state.advance(1);
                            SwiftSyntaxKind::GreaterEqual
                        }
                        Some('>') => {
                            state.advance(1);
                            if let Some('=') = state.peek() {
                                state.advance(1);
                                SwiftSyntaxKind::RightShiftAssign
                            }
                            else {
                                SwiftSyntaxKind::RightShift
                            }
                        }
                        _ => SwiftSyntaxKind::Greater,
                    }
                }
                '&' => {
                    state.advance(1);
                    match state.peek() {
                        Some('&') => {
                            state.advance(1);
                            SwiftSyntaxKind::LogicalAnd
                        }
                        Some('=') => {
                            state.advance(1);
                            SwiftSyntaxKind::AndAssign
                        }
                        _ => SwiftSyntaxKind::BitAnd,
                    }
                }
                '|' => {
                    state.advance(1);
                    match state.peek() {
                        Some('|') => {
                            state.advance(1);
                            SwiftSyntaxKind::LogicalOr
                        }
                        Some('=') => {
                            state.advance(1);
                            SwiftSyntaxKind::OrAssign
                        }
                        _ => SwiftSyntaxKind::BitOr,
                    }
                }
                '^' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        SwiftSyntaxKind::XorAssign
                    }
                    else {
                        SwiftSyntaxKind::BitXor
                    }
                }
                '~' => {
                    state.advance(1);
                    SwiftSyntaxKind::BitNot
                }
                '?' => {
                    state.advance(1);
                    if let Some('?') = state.peek() {
                        state.advance(1);
                        SwiftSyntaxKind::QuestionQuestion
                    }
                    else {
                        SwiftSyntaxKind::Question
                    }
                }
                '.' => {
                    state.advance(1);
                    match state.peek() {
                        Some('.') => {
                            state.advance(1);
                            if let Some('<') = state.peek() {
                                state.advance(1);
                                SwiftSyntaxKind::Range
                            }
                            else {
                                SwiftSyntaxKind::ClosedRange
                            }
                        }
                        _ => SwiftSyntaxKind::Dot,
                    }
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
    fn lex_delimiter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => SwiftSyntaxKind::LeftParen,
                ')' => SwiftSyntaxKind::RightParen,
                '[' => SwiftSyntaxKind::LeftBracket,
                ']' => SwiftSyntaxKind::RightBracket,
                '{' => SwiftSyntaxKind::LeftBrace,
                '}' => SwiftSyntaxKind::RightBrace,
                ',' => SwiftSyntaxKind::Comma,
                ';' => SwiftSyntaxKind::Semicolon,
                ':' => SwiftSyntaxKind::Colon,
                '@' => SwiftSyntaxKind::At,
                '#' => SwiftSyntaxKind::Hash,
                '$' => SwiftSyntaxKind::Dollar,
                '_' => SwiftSyntaxKind::Underscore,
                '\\' => SwiftSyntaxKind::Backslash,
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

impl<'config> Lexer<SwiftLanguage> for SwiftLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, text: &'a S, _edits: &[oak_core::TextEdit], cache: &'a mut impl oak_core::lexer::LexerCache<SwiftLanguage>) -> LexOutput<SwiftLanguage> {
        let mut state = State::new(text);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> SwiftLexer<'config> {
    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            // 尝试各种词法规则
            if self.skip_whitespace(state) {
                continue;
            }

            if self.lex_newline(state) {
                continue;
            }

            if self.lex_comment(state) {
                continue;
            }

            if self.lex_string_literal(state) {
                continue;
            }

            if self.lex_number_literal(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_operator(state) {
                continue;
            }

            if self.lex_delimiter(state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(SwiftSyntaxKind::Error, start_pos, state.get_position());
            }

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }
}
