#![doc = include_str!("readme.md")]
pub mod token_type;
use crate::language::PhpLanguage;
use oak_core::{Lexer, LexerCache, LexerState, OakError, lexer::LexOutput, source::Source};
pub use token_type::{PhpToken, PhpTokenType};

type State<'s, S> = LexerState<'s, S, PhpLanguage>;

/// Lexer for the PHP language.
///
/// This lexer transforms a source string into a stream of [`PhpTokenType`] tokens.
#[derive(Clone, Debug)]
pub struct PhpLexer<'config> {
    _config: &'config PhpLanguage,
}

impl<'config> Lexer<PhpLanguage> for PhpLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[oak_core::source::TextEdit], cache: &'a mut impl LexerCache<PhpLanguage>) -> LexOutput<PhpLanguage> {
        let mut state = State::new_with_cache(source, 0, cache);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> PhpLexer<'config> {
    /// Creates a new `PhpLexer` with the given language configuration.
    pub fn new(config: &'config PhpLanguage) -> Self {
        Self { _config: config }
    }

    fn run<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> Result<(), OakError> {
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
                state.add_token(PhpTokenType::Error, start_pos, state.get_position())
            }
            else {
                // 如果已到达文件末尾，退出循环
                break;
            }
        }

        Ok(())
    }

    fn skip_whitespace<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' {
                state.advance(ch.len_utf8())
            }
            else {
                break;
            }
        }

        if state.get_position() > start_pos {
            state.add_token(PhpTokenType::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_newline<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(PhpTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1)
            }
            state.add_token(PhpTokenType::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_comment<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            state.advance(1);
            if let Some('/') = state.peek() {
                state.advance(1);
                // 单行注释
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8())
                }
                state.add_token(PhpTokenType::Comment, start_pos, state.get_position());
                return true;
            }
            else if let Some('*') = state.peek() {
                state.advance(1);
                // 多行注释
                while let Some(ch) = state.peek() {
                    if ch == '*' {
                        state.advance(1);
                        if let Some('/') = state.peek() {
                            state.advance(1);
                            break;
                        }
                    }
                    else {
                        state.advance(ch.len_utf8())
                    }
                }
                state.add_token(PhpTokenType::Comment, start_pos, state.get_position());
                return true;
            }
            else {
                // 回退，这不是注释
                state.set_position(start_pos);
                return false;
            }
        }
        else if let Some('#') = state.peek() {
            state.advance(1);
            // PHP 风格的单行注释
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8())
            }
            state.add_token(PhpTokenType::Comment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_string<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(quote_char) = state.peek() {
            if quote_char == '"' || quote_char == '\'' {
                state.advance(1); // 跳过开始引号

                let mut escaped = false;
                while let Some(ch) = state.peek() {
                    if escaped {
                        escaped = false;
                        state.advance(ch.len_utf8())
                    }
                    else if ch == '\\' {
                        escaped = true;
                        state.advance(1)
                    }
                    else if ch == quote_char {
                        state.advance(1); // 跳过结束引号
                        break;
                    }
                    else if ch == '\n' || ch == '\r' {
                        // 字符串不能跨行（除非转义）
                        break;
                    }
                    else {
                        state.advance(ch.len_utf8())
                    }
                }

                state.add_token(PhpTokenType::StringLiteral, start_pos, state.get_position());
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

    fn lex_number<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                let start_pos = state.get_position();

                // 读取整数部分
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1)
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
                            state.advance(1)
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
                                state.advance(1)
                            }
                        }
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_digit() {
                                state.advance(1)
                            }
                            else {
                                break;
                            }
                        }
                    }
                }

                state.add_token(PhpTokenType::NumberLiteral, start_pos, state.get_position());
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

    fn lex_identifier_or_keyword<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' || ch == '$' {
                let start_pos = state.get_position();

                // 读取标识符
                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' || ch == '$' {
                        state.advance(ch.len_utf8())
                    }
                    else {
                        break;
                    }
                }

                let end_pos = state.get_position();
                let text = state.source().get_text_in(oak_core::Range { start: start_pos, end: end_pos });

                // 检查是否是关键字
                let kind = match text.as_ref() {
                    "abstract" => PhpTokenType::Abstract,
                    "and" => PhpTokenType::And,
                    "array" => PhpTokenType::Array,
                    "as" => PhpTokenType::As,
                    "break" => PhpTokenType::Break,
                    "callable" => PhpTokenType::Callable,
                    "case" => PhpTokenType::Case,
                    "catch" => PhpTokenType::Catch,
                    "class" => PhpTokenType::Class,
                    "clone" => PhpTokenType::Clone,
                    "const" => PhpTokenType::Const,
                    "continue" => PhpTokenType::Continue,
                    "declare" => PhpTokenType::Declare,
                    "default" => PhpTokenType::Default,
                    "die" => PhpTokenType::Exit,
                    "do" => PhpTokenType::Do,
                    "echo" => PhpTokenType::Echo,
                    "else" => PhpTokenType::Else,
                    "elseif" => PhpTokenType::Elseif,
                    "empty" => PhpTokenType::Empty,
                    "enddeclare" => PhpTokenType::Enddeclare,
                    "endfor" => PhpTokenType::Endfor,
                    "endforeach" => PhpTokenType::Endforeach,
                    "endif" => PhpTokenType::Endif,
                    "endswitch" => PhpTokenType::Endswitch,
                    "endwhile" => PhpTokenType::Endwhile,
                    "eval" => PhpTokenType::Eval,
                    "exit" => PhpTokenType::Exit,
                    "extends" => PhpTokenType::Extends,
                    "final" => PhpTokenType::Final,
                    "finally" => PhpTokenType::Finally,
                    "for" => PhpTokenType::For,
                    "foreach" => PhpTokenType::Foreach,
                    "function" => PhpTokenType::Function,
                    "global" => PhpTokenType::Global,
                    "goto" => PhpTokenType::Goto,
                    "if" => PhpTokenType::If,
                    "implements" => PhpTokenType::Implements,
                    "include" => PhpTokenType::Include,
                    "include_once" => PhpTokenType::IncludeOnce,
                    "instanceof" => PhpTokenType::Instanceof,
                    "insteadof" => PhpTokenType::Insteadof,
                    "interface" => PhpTokenType::Interface,
                    "isset" => PhpTokenType::Isset,
                    "list" => PhpTokenType::List,
                    "namespace" => PhpTokenType::Namespace,
                    "new" => PhpTokenType::New,
                    "or" => PhpTokenType::Or,
                    "print" => PhpTokenType::Print,
                    "private" => PhpTokenType::Private,
                    "protected" => PhpTokenType::Protected,
                    "public" => PhpTokenType::Public,
                    "require" => PhpTokenType::Require,
                    "require_once" => PhpTokenType::RequireOnce,
                    "return" => PhpTokenType::Return,
                    "static" => PhpTokenType::Static,
                    "switch" => PhpTokenType::Switch,
                    "throw" => PhpTokenType::Throw,
                    "trait" => PhpTokenType::Trait,
                    "try" => PhpTokenType::Try,
                    "unset" => PhpTokenType::Unset,
                    "use" => PhpTokenType::Use,
                    "var" => PhpTokenType::Var,
                    "while" => PhpTokenType::While,
                    "xor" => PhpTokenType::Xor,
                    "yield" => PhpTokenType::Yield,
                    "true" => PhpTokenType::BooleanLiteral,
                    "false" => PhpTokenType::BooleanLiteral,
                    "null" => PhpTokenType::NullLiteral,
                    _ => PhpTokenType::Identifier,
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
                        PhpTokenType::Increment
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        PhpTokenType::PlusAssign
                    }
                    else {
                        PhpTokenType::Plus
                    }
                }
                '-' => {
                    state.advance(1);
                    if let Some('-') = state.peek() {
                        state.advance(1);
                        PhpTokenType::Decrement
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        PhpTokenType::MinusAssign
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        PhpTokenType::Arrow
                    }
                    else {
                        PhpTokenType::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    if let Some('*') = state.peek() {
                        state.advance(1);
                        PhpTokenType::Power
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        PhpTokenType::MultiplyAssign
                    }
                    else {
                        PhpTokenType::Multiply
                    }
                }
                '/' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PhpTokenType::DivideAssign
                    }
                    else {
                        PhpTokenType::Divide
                    }
                }
                '%' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PhpTokenType::ModuloAssign
                    }
                    else {
                        PhpTokenType::Modulo
                    }
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            PhpTokenType::Identical
                        }
                        else {
                            PhpTokenType::Equal
                        }
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        PhpTokenType::DoubleArrow
                    }
                    else {
                        PhpTokenType::Assign
                    }
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            PhpTokenType::NotIdentical
                        }
                        else {
                            PhpTokenType::NotEqual
                        }
                    }
                    else {
                        PhpTokenType::LogicalNot
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PhpTokenType::LessEqual
                    }
                    else if let Some('<') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            PhpTokenType::LeftShiftAssign
                        }
                        else {
                            PhpTokenType::LeftShift
                        }
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        PhpTokenType::Spaceship
                    }
                    else {
                        PhpTokenType::Less
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PhpTokenType::GreaterEqual
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            PhpTokenType::RightShiftAssign
                        }
                        else {
                            PhpTokenType::RightShift
                        }
                    }
                    else {
                        PhpTokenType::Greater
                    }
                }
                '&' => {
                    state.advance(1);
                    if let Some('&') = state.peek() {
                        state.advance(1);
                        PhpTokenType::LogicalAnd
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        PhpTokenType::BitwiseAndAssign
                    }
                    else {
                        PhpTokenType::BitwiseAnd
                    }
                }
                '|' => {
                    state.advance(1);
                    if let Some('|') = state.peek() {
                        state.advance(1);
                        PhpTokenType::LogicalOr
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        PhpTokenType::BitwiseOrAssign
                    }
                    else {
                        PhpTokenType::BitwiseOr
                    }
                }
                '^' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PhpTokenType::BitwiseXorAssign
                    }
                    else {
                        PhpTokenType::BitwiseXor
                    }
                }
                '~' => {
                    state.advance(1);
                    PhpTokenType::BitwiseNot
                }
                '?' => {
                    state.advance(1);
                    if let Some('?') = state.peek() {
                        state.advance(1);
                        PhpTokenType::NullCoalesce
                    }
                    else {
                        PhpTokenType::Question
                    }
                }
                ':' => {
                    state.advance(1);
                    if let Some(':') = state.peek() {
                        state.advance(1);
                        PhpTokenType::DoubleColon
                    }
                    else {
                        PhpTokenType::Colon
                    }
                }
                ';' => {
                    state.advance(1);
                    PhpTokenType::Semicolon
                }
                ',' => {
                    state.advance(1);
                    PhpTokenType::Comma
                }
                '.' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PhpTokenType::ConcatAssign
                    }
                    else {
                        PhpTokenType::Dot
                    }
                }
                '(' => {
                    state.advance(1);
                    PhpTokenType::LeftParen
                }
                ')' => {
                    state.advance(1);
                    PhpTokenType::RightParen
                }
                '[' => {
                    state.advance(1);
                    PhpTokenType::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    PhpTokenType::RightBracket
                }
                '{' => {
                    state.advance(1);
                    PhpTokenType::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    PhpTokenType::RightBrace
                }
                '$' => {
                    state.advance(1);
                    PhpTokenType::Dollar
                }
                '@' => {
                    state.advance(1);
                    PhpTokenType::At
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
