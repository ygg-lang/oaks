#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::JavaLanguage, lexer::token_type::JavaTokenType};
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
                state.add_token(JavaTokenType::Error, start_pos, state.get_position());
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
            state.add_token(JavaTokenType::Whitespace, start, state.get_position());
            return true;
        }
        false
    }

    /// 处理换行
    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(JavaTokenType::Whitespace, start, state.get_position());
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
            state.add_token(JavaTokenType::LineComment, start, state.get_position());
            return true;
        }

        // 多行注释 /* */
        if state.peek() == Some('/') && state.peek_next_n(1) == Some('*') {
            let start = state.get_position();
            state.advance(2);
            while let Some(ch) = state.peek() {
                if ch == '*' && state.peek_next_n(1) == Some('/') {
                    state.advance(2);
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(JavaTokenType::BlockComment, start, state.get_position());
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
                    // 未闭合的字符串
                    break;
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            state.add_token(JavaTokenType::StringLiteral, start, state.get_position());
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

            state.add_token(JavaTokenType::CharacterLiteral, start, state.get_position());
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

                let text = state.get_text_in((start..state.get_position()).into());
                let kind = if text.contains('.') || text.contains('e') || text.contains('E') || text.ends_with('f') || text.ends_with('F') || text.ends_with('d') || text.ends_with('D') {
                    JavaTokenType::FloatingPointLiteral
                }
                else {
                    JavaTokenType::IntegerLiteral
                };

                eprintln!("DEBUG: Lexer classified '{}' as {:?} at {}..{}", text, kind, start, state.get_position());
                state.add_token(kind, start, state.get_position());
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

                eprintln!("DEBUG: Lexer classified '{}' as {:?} at {}..{}", text, token_kind, start, state.get_position());
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
    fn classify_identifier(&self, text: &str) -> JavaTokenType {
        match text {
            "abstract" => JavaTokenType::Abstract,
            "assert" => JavaTokenType::Assert,
            "boolean" => JavaTokenType::Boolean,
            "break" => JavaTokenType::Break,
            "byte" => JavaTokenType::Byte,
            "case" => JavaTokenType::Case,
            "catch" => JavaTokenType::Catch,
            "char" => JavaTokenType::Char,
            "class" => JavaTokenType::Class,
            "const" => JavaTokenType::Const,
            "continue" => JavaTokenType::Continue,
            "default" => JavaTokenType::Default,
            "do" => JavaTokenType::Do,
            "double" => JavaTokenType::Double,
            "else" => JavaTokenType::Else,
            "enum" => JavaTokenType::Enum,
            "extends" => JavaTokenType::Extends,
            "final" => JavaTokenType::Final,
            "finally" => JavaTokenType::Finally,
            "float" => JavaTokenType::Float,
            "for" => JavaTokenType::For,
            "goto" => JavaTokenType::Goto,
            "if" => JavaTokenType::If,
            "implements" => JavaTokenType::Implements,
            "import" => JavaTokenType::Import,
            "instanceof" => JavaTokenType::Instanceof,
            "int" => JavaTokenType::Int,
            "interface" => JavaTokenType::Interface,
            "long" => JavaTokenType::Long,
            "native" => JavaTokenType::Native,
            "new" => JavaTokenType::New,
            "package" => JavaTokenType::Package,
            "private" => JavaTokenType::Private,
            "protected" => JavaTokenType::Protected,
            "public" => JavaTokenType::Public,
            "record" => JavaTokenType::Record,
            "return" => JavaTokenType::Return,
            "short" => JavaTokenType::Short,
            "static" => JavaTokenType::Static,
            "strictfp" => JavaTokenType::Strictfp,
            "struct" => JavaTokenType::Struct,
            "super" => JavaTokenType::Super,
            "switch" => JavaTokenType::Switch,
            "synchronized" => JavaTokenType::Synchronized,
            "this" => JavaTokenType::This,
            "throw" => JavaTokenType::Throw,
            "throws" => JavaTokenType::Throws,
            "transient" => JavaTokenType::Transient,
            "try" => JavaTokenType::Try,
            "void" => JavaTokenType::Void,
            "volatile" => JavaTokenType::Volatile,
            "while" => JavaTokenType::While,
            "true" | "false" => JavaTokenType::BooleanLiteral,
            "null" => JavaTokenType::NullLiteral,
            _ => JavaTokenType::Identifier,
        }
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
                        JavaTokenType::PlusPlus
                    }
                    else if state.peek() == Some('=') {
                        state.advance(1);
                        JavaTokenType::PlusEquals
                    }
                    else {
                        JavaTokenType::Plus
                    }
                }
                '-' => {
                    state.advance(1);
                    if state.peek() == Some('-') {
                        state.advance(1);
                        JavaTokenType::MinusMinus
                    }
                    else if state.peek() == Some('=') {
                        state.advance(1);
                        JavaTokenType::MinusEquals
                    }
                    else {
                        JavaTokenType::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        JavaTokenType::AsteriskEquals
                    }
                    else {
                        JavaTokenType::Asterisk
                    }
                }
                '/' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        JavaTokenType::SlashEquals
                    }
                    else {
                        JavaTokenType::Slash
                    }
                }
                '%' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        JavaTokenType::PercentEquals
                    }
                    else {
                        JavaTokenType::Percent
                    }
                }
                '=' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        JavaTokenType::Equals
                    }
                    else {
                        JavaTokenType::Assign
                    }
                }
                '!' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        JavaTokenType::BangEquals
                    }
                    else {
                        JavaTokenType::Bang
                    }
                }
                '<' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        JavaTokenType::LessThanEquals
                    }
                    else if state.peek() == Some('<') {
                        state.advance(1);
                        if state.peek() == Some('=') {
                            state.advance(1);
                            JavaTokenType::LeftShiftEquals
                        }
                        else {
                            JavaTokenType::LeftShift
                        }
                    }
                    else {
                        JavaTokenType::LessThan
                    }
                }
                '>' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        JavaTokenType::GreaterThanEquals
                    }
                    else if state.peek() == Some('>') {
                        state.advance(1);
                        if state.peek() == Some('>') {
                            state.advance(1);
                            if state.peek() == Some('=') {
                                state.advance(1);
                                JavaTokenType::UnsignedRightShiftEquals
                            }
                            else {
                                JavaTokenType::UnsignedRightShift
                            }
                        }
                        else if state.peek() == Some('=') {
                            state.advance(1);
                            JavaTokenType::RightShiftEquals
                        }
                        else {
                            JavaTokenType::RightShift
                        }
                    }
                    else {
                        JavaTokenType::GreaterThan
                    }
                }
                '&' => {
                    state.advance(1);
                    if state.peek() == Some('&') {
                        state.advance(1);
                        JavaTokenType::AmpersandAmpersand
                    }
                    else if state.peek() == Some('=') {
                        state.advance(1);
                        JavaTokenType::AmpersandEquals
                    }
                    else {
                        JavaTokenType::Ampersand
                    }
                }
                '|' => {
                    state.advance(1);
                    if state.peek() == Some('|') {
                        state.advance(1);
                        JavaTokenType::PipePipe
                    }
                    else if state.peek() == Some('=') {
                        state.advance(1);
                        JavaTokenType::PipeEquals
                    }
                    else {
                        JavaTokenType::Pipe
                    }
                }
                '^' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        JavaTokenType::CaretEquals
                    }
                    else {
                        JavaTokenType::Caret
                    }
                }
                '~' => {
                    state.advance(1);
                    JavaTokenType::Tilde
                }
                '?' => {
                    state.advance(1);
                    JavaTokenType::Question
                }
                ':' => {
                    state.advance(1);
                    JavaTokenType::Colon
                }
                ';' => {
                    state.advance(1);
                    JavaTokenType::Semicolon
                }
                ',' => {
                    state.advance(1);
                    JavaTokenType::Comma
                }
                '.' => {
                    state.advance(1);
                    if state.peek() == Some('.') && state.peek_next_n(1) == Some('.') {
                        state.advance(2);
                        JavaTokenType::Ellipsis
                    }
                    else {
                        JavaTokenType::Dot
                    }
                }
                '(' => {
                    state.advance(1);
                    JavaTokenType::LeftParen
                }
                ')' => {
                    state.advance(1);
                    JavaTokenType::RightParen
                }
                '{' => {
                    state.advance(1);
                    JavaTokenType::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    JavaTokenType::RightBrace
                }
                '[' => {
                    state.advance(1);
                    JavaTokenType::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    JavaTokenType::RightBracket
                }
                '@' => {
                    state.advance(1);
                    JavaTokenType::At
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
