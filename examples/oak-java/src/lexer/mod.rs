use crate::{kind::JavaSyntaxKind, language::JavaLanguage};
use oak_core::{Lexer, LexerCache, LexerState, OakError, lexer::LexOutput, source::Source};

type State<'a, S> = LexerState<'a, S, JavaLanguage>;

#[derive(Clone, Debug)]
pub struct JavaLexer<'config> {
    _config: &'config JavaLanguage,
}

impl<'config> Lexer<JavaLanguage> for JavaLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[oak_core::source::TextEdit], cache: &'a mut impl LexerCache<JavaLanguage>) -> LexOutput<JavaLanguage> {
        let mut state = State::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> JavaLexer<'config> {
    pub fn new(config: &'config JavaLanguage) -> Self {
        Self { _config: config }
    }

    /// 主要的词法分析循环
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

            if self.lex_char_literal(state) {
                continue;
            }

            if self.lex_number_literal(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_operator_or_delimiter(state) {
                continue;
            }

            // 如果没有匹配到任何规则，前进一个字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(JavaSyntaxKind::Error, start_pos, state.get_position());
            }

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }

    /// 跳过空白字符（不包括换行符）
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' || ch == '\r' {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        if state.get_position() > start {
            state.add_token(JavaSyntaxKind::Whitespace, start, state.get_position());
            return true;
        }
        false
    }

    /// 处理换行
    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(JavaSyntaxKind::Whitespace, start, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 跳过注释
    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        // 单行注释 //
        if state.peek() == Some('/') && state.peek_next_n(1) == Some('/') {
            state.advance(2);
            while let Some(ch) = state.peek() {
                if ch == '\n' {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(JavaSyntaxKind::LineComment, start, state.get_position());
            return true;
        }

        // 多行注释 /* */
        if state.peek() == Some('/') && state.peek_next_n(1) == Some('*') {
            state.advance(2);
            while let Some(ch) = state.peek() {
                if ch == '*' && state.peek_next_n(1) == Some('/') {
                    state.advance(2);
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(JavaSyntaxKind::BlockComment, start, state.get_position());
            return true;
        }

        false
    }

    /// 处理字符串字面量
    fn lex_string_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some('"') = state.peek() {
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
                else if ch == '\n' {
                    // 未闭合的字符�?
                    break;
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            state.add_token(JavaSyntaxKind::StringLiteral, start, state.get_position());
            return true;
        }

        false
    }

    /// 处理字符字面�?
    fn lex_char_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some('\'') = state.peek() {
            state.advance(1);

            if let Some(ch) = state.peek() {
                if ch == '\\' {
                    state.advance(1);
                    if let Some(escaped) = state.peek() {
                        state.advance(escaped.len_utf8());
                    }
                }
                else if ch != '\'' && ch != '\n' {
                    state.advance(ch.len_utf8());
                }
            }

            if let Some('\'') = state.peek() {
                state.advance(1);
            }

            state.add_token(JavaSyntaxKind::CharacterLiteral, start, state.get_position());
            return true;
        }

        false
    }

    /// 处理数字字面�?
    fn lex_number_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                // 处理整数部分
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 处理小数部分
                if state.peek() == Some('.') && state.peek_next_n(1).map_or(false, |c| c.is_ascii_digit()) {
                    state.advance(1); // '.'
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(ch.len_utf8());
                        }
                        else {
                            break;
                        }
                    }
                }

                // 处理指数部分
                if let Some(ch) = state.peek() {
                    if ch == 'e' || ch == 'E' {
                        state.advance(1);
                        if let Some(sign) = state.peek() {
                            if sign == '+' || sign == '-' {
                                state.advance(1);
                            }
                        }
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_digit() {
                                state.advance(ch.len_utf8());
                            }
                            else {
                                break;
                            }
                        }
                    }
                }

                // 处理后缀
                if let Some(suffix) = state.peek() {
                    if suffix == 'f' || suffix == 'F' || suffix == 'd' || suffix == 'D' || suffix == 'l' || suffix == 'L' {
                        state.advance(1);
                    }
                }

                state.add_token(JavaSyntaxKind::IntegerLiteral, start, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理标识符或关键�?
    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' || ch == '$' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '$' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = state.get_text_in((start..state.get_position()).into());
                let token_kind = self.classify_identifier(text.as_ref());

                state.add_token(token_kind, start, state.get_position());
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

    /// 分类标识符为关键字或普通标识符
    fn classify_identifier(&self, text: &str) -> JavaSyntaxKind {
        let kind = match text {
            "abstract" => JavaSyntaxKind::Abstract,
            "assert" => JavaSyntaxKind::Assert,
            "boolean" => JavaSyntaxKind::Boolean,
            "break" => JavaSyntaxKind::Break,
            "byte" => JavaSyntaxKind::Byte,
            "case" => JavaSyntaxKind::Case,
            "catch" => JavaSyntaxKind::Catch,
            "char" => JavaSyntaxKind::Char,
            "class" => JavaSyntaxKind::Class,
            "const" => JavaSyntaxKind::Const,
            "continue" => JavaSyntaxKind::Continue,
            "default" => JavaSyntaxKind::Default,
            "do" => JavaSyntaxKind::Do,
            "double" => JavaSyntaxKind::Double,
            "else" => JavaSyntaxKind::Else,
            "enum" => JavaSyntaxKind::Enum,
            "extends" => JavaSyntaxKind::Extends,
            "final" => JavaSyntaxKind::Final,
            "finally" => JavaSyntaxKind::Finally,
            "float" => JavaSyntaxKind::Float,
            "for" => JavaSyntaxKind::For,
            "goto" => JavaSyntaxKind::Goto,
            "if" => JavaSyntaxKind::If,
            "implements" => JavaSyntaxKind::Implements,
            "import" => JavaSyntaxKind::Import,
            "instanceof" => JavaSyntaxKind::Instanceof,
            "int" => JavaSyntaxKind::Int,
            "interface" => JavaSyntaxKind::Interface,
            "long" => JavaSyntaxKind::Long,
            "native" => JavaSyntaxKind::Native,
            "new" => JavaSyntaxKind::New,
            "package" => JavaSyntaxKind::Package,
            "private" => JavaSyntaxKind::Private,
            "protected" => JavaSyntaxKind::Protected,
            "public" => JavaSyntaxKind::Public,
            "return" => JavaSyntaxKind::Return,
            "short" => JavaSyntaxKind::Short,
            "static" => JavaSyntaxKind::Static,
            "strictfp" => JavaSyntaxKind::Strictfp,
            "super" => JavaSyntaxKind::Super,
            "switch" => JavaSyntaxKind::Switch,
            "synchronized" => JavaSyntaxKind::Synchronized,
            "this" => JavaSyntaxKind::This,
            "throw" => JavaSyntaxKind::Throw,
            "throws" => JavaSyntaxKind::Throws,
            "transient" => JavaSyntaxKind::Transient,
            "try" => JavaSyntaxKind::Try,
            "void" => JavaSyntaxKind::Void,
            "volatile" => JavaSyntaxKind::Volatile,
            "while" => JavaSyntaxKind::While,
            "true" | "false" => JavaSyntaxKind::BooleanLiteral,
            "null" => JavaSyntaxKind::NullLiteral,
            _ => JavaSyntaxKind::Identifier,
        };
        eprintln!("DEBUG: Lexer classified '{}' as {:?}", text, kind);
        kind
    }

    /// 处理操作符和分隔�?
    fn lex_operator_or_delimiter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '+' => {
                    state.advance(1);
                    if state.peek() == Some('+') {
                        state.advance(1);
                        JavaSyntaxKind::PlusPlus
                    }
                    else if state.peek() == Some('=') {
                        state.advance(1);
                        JavaSyntaxKind::PlusEquals
                    }
                    else {
                        JavaSyntaxKind::Plus
                    }
                }
                '-' => {
                    state.advance(1);
                    if state.peek() == Some('-') {
                        state.advance(1);
                        JavaSyntaxKind::MinusMinus
                    }
                    else if state.peek() == Some('=') {
                        state.advance(1);
                        JavaSyntaxKind::MinusEquals
                    }
                    else {
                        JavaSyntaxKind::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        JavaSyntaxKind::AsteriskEquals
                    }
                    else {
                        JavaSyntaxKind::Asterisk
                    }
                }
                '/' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        JavaSyntaxKind::SlashEquals
                    }
                    else {
                        JavaSyntaxKind::Slash
                    }
                }
                '%' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        JavaSyntaxKind::PercentEquals
                    }
                    else {
                        JavaSyntaxKind::Percent
                    }
                }
                '=' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        JavaSyntaxKind::Equals
                    }
                    else {
                        JavaSyntaxKind::Assign
                    }
                }
                '!' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        JavaSyntaxKind::BangEquals
                    }
                    else {
                        JavaSyntaxKind::Bang
                    }
                }
                '<' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        JavaSyntaxKind::LessThanEquals
                    }
                    else if state.peek() == Some('<') {
                        state.advance(1);
                        if state.peek() == Some('=') {
                            state.advance(1);
                            JavaSyntaxKind::LeftShiftEquals
                        }
                        else {
                            JavaSyntaxKind::LeftShift
                        }
                    }
                    else {
                        JavaSyntaxKind::LessThan
                    }
                }
                '>' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        JavaSyntaxKind::GreaterThanEquals
                    }
                    else if state.peek() == Some('>') {
                        state.advance(1);
                        if state.peek() == Some('>') {
                            state.advance(1);
                            if state.peek() == Some('=') {
                                state.advance(1);
                                JavaSyntaxKind::UnsignedRightShiftEquals
                            }
                            else {
                                JavaSyntaxKind::UnsignedRightShift
                            }
                        }
                        else if state.peek() == Some('=') {
                            state.advance(1);
                            JavaSyntaxKind::RightShiftEquals
                        }
                        else {
                            JavaSyntaxKind::RightShift
                        }
                    }
                    else {
                        JavaSyntaxKind::GreaterThan
                    }
                }
                '&' => {
                    state.advance(1);
                    if state.peek() == Some('&') {
                        state.advance(1);
                        JavaSyntaxKind::AmpersandAmpersand
                    }
                    else if state.peek() == Some('=') {
                        state.advance(1);
                        JavaSyntaxKind::AmpersandEquals
                    }
                    else {
                        JavaSyntaxKind::Ampersand
                    }
                }
                '|' => {
                    state.advance(1);
                    if state.peek() == Some('|') {
                        state.advance(1);
                        JavaSyntaxKind::PipePipe
                    }
                    else if state.peek() == Some('=') {
                        state.advance(1);
                        JavaSyntaxKind::PipeEquals
                    }
                    else {
                        JavaSyntaxKind::Pipe
                    }
                }
                '^' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        JavaSyntaxKind::CaretEquals
                    }
                    else {
                        JavaSyntaxKind::Caret
                    }
                }
                '~' => {
                    state.advance(1);
                    JavaSyntaxKind::Tilde
                }
                '?' => {
                    state.advance(1);
                    JavaSyntaxKind::Question
                }
                ':' => {
                    state.advance(1);
                    JavaSyntaxKind::Colon
                }
                ';' => {
                    state.advance(1);
                    JavaSyntaxKind::Semicolon
                }
                ',' => {
                    state.advance(1);
                    JavaSyntaxKind::Comma
                }
                '.' => {
                    state.advance(1);
                    if state.peek() == Some('.') && state.peek_next_n(1) == Some('.') {
                        state.advance(2);
                        JavaSyntaxKind::Ellipsis
                    }
                    else {
                        JavaSyntaxKind::Dot
                    }
                }
                '(' => {
                    state.advance(1);
                    JavaSyntaxKind::LeftParen
                }
                ')' => {
                    state.advance(1);
                    JavaSyntaxKind::RightParen
                }
                '{' => {
                    state.advance(1);
                    JavaSyntaxKind::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    JavaSyntaxKind::RightBrace
                }
                '[' => {
                    state.advance(1);
                    JavaSyntaxKind::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    JavaSyntaxKind::RightBracket
                }
                '@' => {
                    state.advance(1);
                    JavaSyntaxKind::At
                }
                _ => return false,
            };

            state.add_token(token_kind, start, state.get_position());
            true
        }
        else {
            false
        }
    }
}
